use actix_web::web::Buf;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sled::Tree;

use crate::{api::types::ImageUrl, config::Config};

pub mod user;

pub type User = uuid::Uuid;
pub type Game = uuid::Uuid;
pub type Ruleset = uuid::Uuid;
pub type Setting = uuid::Uuid;


fn get_data<'a, T, K>(db: &Tree, key: &K) -> Result<Option<T>, Error> 
where 
    T: DeserializeOwned, 
    K: std::convert::AsRef<[u8]>
{
    if let Some(data) = db.get(&key)? {
        Ok(bincode::deserialize_from(data.reader())?)
    } else {
        Ok(None)
    }
}

#[derive(Debug)]
pub struct Empty;

#[derive(Debug)]
pub enum Error<C = Empty> {
    DbConflict(C),
    DbErr(sled::Error),
    ParseErr(bincode::Error),
    Bcrypt(bcrypt::BcryptError),
    Other(String)
}

pub enum UpdateResponse {
    Success,
    NotFound,
}

impl<T> From<bincode::Error> for Error<T> {
    fn from(value: bincode::Error) -> Self {
        Error::ParseErr(value)
    }
}

impl<T> From<sled::Error> for Error<T> {
    fn from(value: sled::Error) -> Self {
        Error::DbErr(value)
    }
}

impl<T> From<bcrypt::BcryptError> for Error<T> {
    fn from(value: bcrypt::BcryptError) -> Self {
        Error::Bcrypt(value)
    }
}

impl ImageUrl {
    pub fn to_string(self, config: &Config) -> String {
        match self {
            ImageUrl::ExternalPath(path) => path,
            ImageUrl::InternalServerPath(path) => format!("http://{}:{}/{}", config.server.host, config.server.port, path),
        }
    }

    pub fn sanatize(self) -> Result<Self, Error> {
        todo!()
    }
    // pub fn from_str(s: &str, user: User, db: &Database, config: &Config) -> Option<ImageUrl> {
    //     // TODO: Look at user's uploaded data and see if the file path given exists there
    //     // If it does, use that as an internal server path. If not, check to see if it is a valid
    //     // external path to another server's image. If it is, use it as an external path.
    //     // If it is neither, return None
    //     todo!()
    // }
}
pub struct Database {
    pub user_db: user::UserDB,
}

impl Database {
    pub fn open(config: &Config) -> Database {
        Database {
            user_db: user::UserDB::open(&config).unwrap()
        }
    }
}