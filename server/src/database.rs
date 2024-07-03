use std::{ffi::OsStr, fmt::Debug, fs::File, ops::Deref, path::Path, rc::Rc, sync::Arc};

use actix_multipart::form::tempfile::TempFile;
use actix_web::web::Buf;
use log::info;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sled::Tree;
use user::UserDB;

use crate::{api::{schema::{UserLoginSchema, UserRegistrationSchema, UserUpdateSchema}, types::{ConflictError, ImageData, ImageUrl, InsufficientStorageError, InternalError, NotFoundError, PublicUserData, ServerError, ServerErrorType, UnsupportedError, UploadImageData, UserData}}, config::Config};

pub mod user;

pub type User = uuid::Uuid;
pub type Game = uuid::Uuid;
pub type Ruleset = uuid::Uuid;
pub type Setting = uuid::Uuid;
pub type Character = uuid::Uuid; // Stored under the user's data (user_id/character_id ?)


fn get_data<'a, T, K>(db: &Tree, key: &K) -> Result<Option<T>, Error> 
where 
    T: DeserializeOwned, 
    K: std::convert::AsRef<[u8]> + Debug
{
    if let Some(data) = db.get(&key)? {
        Ok(Some(bincode::deserialize_from(data.reader())?))
    } else {
        Ok(None)
    }
}

// Meant for internal server errors. 
// Don't send the sled::Error, bincode::Error, or bcrypt::BcryptError data back to the client.
#[derive(Debug)]
pub enum Error {
    Database(sled::Error),
    Parse(bincode::Error),
    Bcrypt(bcrypt::BcryptError),
    Filesystem(std::io::Error),
    Other(ServerError)
}

impl From<sled::Error> for Error {
    fn from(value: sled::Error) -> Self {
        Error::Database(value)
    }
}

impl From<bincode::Error> for Error {
    fn from(value: bincode::Error) -> Self {
        Error::Parse(value)
    }
}

impl From<bcrypt::BcryptError> for Error {
    fn from(value: bcrypt::BcryptError) -> Self {
        Error::Bcrypt(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Filesystem(value)
    }
}


impl From<ServerError> for Error {
    fn from(value: ServerError) -> Self {
        Error::Other(value)
    }
}

impl From<Error> for ServerError {
    fn from(value: Error) -> Self {
        match value {
            Error::Database(e) => ServerError { 
                error: ServerErrorType::InternalError(InternalError::Database), 
                message: e.to_string() 
            },
            Error::Parse(e) => ServerError { 
                error: ServerErrorType::InternalError(InternalError::Parse), 
                message: e.to_string() 
            },
            Error::Bcrypt(e) => ServerError { 
                error: ServerErrorType::InternalError(InternalError::Encrypt), 
                message: e.to_string() 
            },
            Error::Filesystem(e) => ServerError { 
                error: ServerErrorType::InternalError(InternalError::Filesystem), 
                message: e.to_string() 
            },
            Error::Other(err) => err,
        }
    }
}

pub fn serverpath_from_filepath(internal_path: &str, config: &Config) -> String {
    format!("http://{}:{}/{}", config.server.host, config.server.port, internal_path)
}

impl ImageUrl {
    pub fn to_string(self, config: &Config) -> String {
        match self {
            ImageUrl::External(s) => s,
            ImageUrl::Internal(internal_path) => serverpath_from_filepath(&internal_path, config),
        }
    }
}

pub struct Database {
    user_db: user::UserDB,
    config: Config 
    // Right now, just cloning config data. Might be better to use an Arc depending on performance. 
    // Currently, the impact will depend on the size of Config and the number of sub-databases
}

impl Database {
    pub fn open(config: Config) -> Database {
        Database {
            user_db: user::UserDB::open(config.clone()).unwrap(),
            config,
        }
    }

    pub fn user_data<'a>(&'a self) -> UserDatabaseHandle<'a> {
        UserDatabaseHandle { 
            db_ref: self
        }
    }
}
// Handles will encapsulate the actions necessary to keep the whole of the sever data consistent.
// For example, when deleting users, it is also important to remove them from all active games
pub struct UserDatabaseHandle<'a> {
    db_ref: &'a Database
}

// This makes it so we only have to implement the methods on UserDatabaseHandle
// when changes actually need to be made from the UserDB generic implementation.
impl<'a> Deref for UserDatabaseHandle<'a> 
{
    type Target = UserDB;

    fn deref(&self) -> &Self::Target {
       &self.db_ref.user_db
    }
}

impl UserDatabaseHandle<'_> {
    pub fn update_user_data(&self, user: User, data: UserUpdateSchema) -> Result<(), Error> {
        let user_db = &self.db_ref.user_db;
        let res = match data {
            UserUpdateSchema::Email(new_email) => user_db.update_email(user, new_email),
            UserUpdateSchema::Password(new_password) => user_db.update_password(user, new_password),
            UserUpdateSchema::ProfileName(profile_name) => user_db.update_profile_name(user, profile_name),
            UserUpdateSchema::ProfileText(profile_text) => user_db.update_profile_text(user, profile_text),
            UserUpdateSchema::ProfileCatchphrase(profile_catchphrase) => user_db.update_profile_catchphrase(user, profile_catchphrase),
            UserUpdateSchema::ProfilePicture(profile_photo) => {
                todo!()
                // match profile_photo.sanatize() {
                //     Ok(url) => user_db.update_profile_photo(user, url),
                //     Err(e) => Err(e),
                // }
            },
            UserUpdateSchema::ProfileBanner(profile_banner) => {
                todo!()
                // match profile_banner.sanatize() {
                //     Ok(url) => user_db.update_profile_banner(user, url),
                //     Err(e) => Err(e),
                // }
            },
            UserUpdateSchema::FavoritedRuleset(id) => todo!(),
            UserUpdateSchema::FavoritedSetting(id) => todo!(),
        };
        todo!()
    }

    pub fn upload(&self, user: User, file: TempFile, name: String) -> Result<(), Error> {
        let user_data = self.get_private_data(user)?;
        if user_data.storage_used + file.size as i64 <= user_data.storage_limit {
            let ext = get_file_ext(file.file_name);
            let filepath = &format!(
                "{}/{}/{}", 
                self.db_ref.config.database.uploads_path,
                user_data.id, // This is b/c username doesn't change
                format!("{}.{}", sanitize_filename(&name), ext)
            );
            let path: &Path = Path::new(filepath);
            if path.exists() {
                return Err(ServerError::new(
                    ServerErrorType::Conflict(ConflictError::FileName),
                    "Filename taken".to_string(),
                ).into());
            }
            if !is_allowed_file_type(path.extension()) {
                info!("Invalid extension: {:?} for path: {:?}", path.extension(), path);
                return Err(ServerError::new(
                    ServerErrorType::Unsupported(UnsupportedError::FileType),
                    "Filetype Unsupported".to_string(),
                ).into());
            }
            // TODO: 
            // [x]. Update the storage of the user
            // [ ]. Update database meta-info on stored files
            // [x]. Reject file types not supported
            //    - Currently, only need to support images, such as jepg, png, gif, ico, svg, etc.
            info!("Attempting to persist: {:?}", path);
            match file.file.persist(filepath) {
                // Might need to handle deleting the file if updating storage fails
                Ok(_) => self.update_storage_usage(user, user_data.storage_used + file.size as i64),
                Err(e) => Err(ServerError {
                    error: ServerErrorType::InternalError(InternalError::Filesystem),
                    message: e.to_string(),
                }.into()),
            }
        } else {
            Err(ServerError {
                error: ServerErrorType::InsufficientStorage(InsufficientStorageError { 
                    current: user_data.storage_used, 
                    maximum: user_data.storage_limit, 
                    given_increase: file.size as i64,
                }),
                message: "Exceeded storage usage".to_string(),
            }.into())
        }
    }

    pub fn get_uploads(&self, user: User) -> Result<Vec<ImageData>, Error> {
        let filepath = &format!(
            "{}/{}", 
            self.db_ref.config.database.uploads_path,
            user,
        );
        if let Ok(paths) = std::fs::read_dir(filepath) {
            let mut res = vec![];
            for path in paths {
                if let Ok(dir) = path {
                    if let Some(img) = image_data_from_path(dir.path().as_path(), &self.db_ref.config) {
                        res.push(img);
                    }
                }
            }
            return Ok(res);
        }
        Err(ServerError::new(
            ServerErrorType::InternalError(InternalError::Filesystem),
            "Failed to read internal files".to_string(),
        ).into())
    }

    pub fn get_upload(&self, user: User, file_name: String) -> Result<ImageData, Error> {
        let filepath = &format!(
            "{}/{}/{}", 
            self.db_ref.config.database.uploads_path,
            user,
            file_name
        );
        if let Some(img) = image_data_from_path(Path::new(filepath), &self.db_ref.config) {
            Ok(img)
        } else {
            Err(ServerError::new(
                ServerErrorType::NotFound(NotFoundError::File(file_name)),
                "Could not find given file".to_string(),
            ).into())
        }
    }
}

///////////////////////////////////////////////////
////////////// Helper Functions ///////////////////
///////////////////////////////////////////////////
fn is_allowed_file_type(t: Option<&OsStr>) -> bool {
    if let Some(t) = t {
        t.eq("png") || t.eq("jgep") || t.eq("gif")
    } else {
        false
    }
}

fn get_file_ext(name: Option<String>) -> String {
    if let Some(name) = name {
        Path::new(&name)        
            .extension().and_then(OsStr::to_str).and_then(|f| Some(f.to_string())).unwrap_or_default()
    } else {
        "".to_string()
    }
}

fn image_data_from_path(p: &Path, config: &Config) -> Option<ImageData> {
    if is_allowed_file_type(p.extension()) {
        if let Ok(file) = File::open(p) {
            return Some(ImageData::InternalUpload(UploadImageData { 
                src: serverpath_from_filepath(p.to_str().unwrap(), config), 
                name: p.file_name()
                        .and_then(|f| f.to_str().and_then(|f| Some(f.to_string()))).unwrap_or_default(), 
                size: file.metadata().unwrap().len() as i64
            }))
        }
    }
    None
}

fn sanitize_filename(name: &str) -> String {
    name.chars().filter(|c| *c != '/' && *c != '\\').collect()
}