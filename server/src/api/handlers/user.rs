use std::{ffi::OsStr, fmt::Debug, fs::{DirEntry, File}, io::Write, path::Path};

use actix_web::{cookie::{time::Duration as ActixWebDuration, Cookie}, get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use log::{info, warn};
use serde_json::json;

use crate::{api::{jwt_auth::{self, TokenClaims}, schema::{FileUploadMetadata, UserLoginSchema, UserRegistrationSchema, UserUpdateSchema}, types::{ ImageData, ImageUrl, LoginError, RegistrationError, ServerError as ServerErrorResponse, UploadError, UserDataError, UserLoginResponse}}, config::Config, database::{serverpath_from_filepath, user::{LoginResponse, RegistrationConflict}, Error}, Database};

use actix_multipart::form::{self, json::Json as MPJson, tempfile::TempFile, MultipartForm};

pub fn setup_routes(cfg: &mut web::ServiceConfig) -> &mut web::ServiceConfig {
    let scope = web::scope("/api")
        .service(get_public_user_handler)
        .service(user_update_handler)
        .service(user_upload_file)
        .service(register_handler)
        .service(login_handler)
        .service(logout_handler)
        .service(get_me_handler);

    cfg.service(scope)
}

#[get("/public/user/{id}")]
async fn get_public_user_handler(
    path: web::Path<uuid::Uuid>,
    db: web::Data<Database>,
    config: web::Data<Config>,
) -> impl Responder {
    let user = path.into_inner();
    match db.user_db.get_public_data(user, &config) {
        Ok(response) => {
            if let Some(data) = response {
                HttpResponse::Ok().json(data)
            } else {
                HttpResponse::NotFound().json(UserDataError::UserIdNotFound(user))
            }
        },
        Err(e) => handle_server_error(e, generic_conflict_handler),
    }
}

#[post("/auth/register")]
async fn register_handler(
    body: web::Json<UserRegistrationSchema>, 
    db: web::Data<Database>,
    config: web::Data<Config>,
) -> impl Responder {
    match db.user_db.register_user(body.into_inner()) {
        Ok(data) => {
            // TODO: Move this to the DATABASE so that it can handle the folders
            let path = &format!(
                "{}/{}", 
                config.database.uploads_path,
                data // This is b/c username doesn't change
            );
            let filepath = Path::new(path);
            if !filepath.exists() {
                if let Err(e) = std::fs::create_dir(filepath) {
                    // Failed to create database folder for user
                    db.user_db.delete_user(data).expect("Failed to delete user just created");
                    return HttpResponse::InternalServerError().json(ServerErrorResponse { 
                        error: "Database Error".to_string(), message: "Failed to create user uploads directory".to_string()
                    });
                }
            }
            HttpResponse::Ok().json(db.user_db.get_private_data(data, &config).unwrap())
        },
        Err(e) => {
            handle_server_error(e, |conflict| {
                match conflict {
                    RegistrationConflict::UsernameTaken => {
                        return HttpResponse::Conflict().json(RegistrationError::EmailTaken);
                    },
                    RegistrationConflict::EmailTaken => {
                        return HttpResponse::Conflict().json(RegistrationError::UsernameTaken);
                    },
                    _ => generic_conflict_handler(conflict),
                }
            })
        },
    }
}

#[post("/auth/login")]
async fn login_handler(
    login: web::Json<UserLoginSchema>, 
    db: web::Data<Database>,
    config: web::Data<Config>
) -> impl Responder {
    // When the user login is successful,
    // the user has secure login. Otherwise, not logged in.
    match db.user_db.login_user(login.into_inner()) {
        Ok(response) => {
            match response {
                LoginResponse::Success(user) => {
                    let now = Utc::now();
                    let iat = now.timestamp() as usize;
                    let exp = (now + Duration::days(config.jwt.expiration)).timestamp() as usize;
                    let claims: TokenClaims = TokenClaims {
                        user_id: user.to_string(),
                        exp,
                        iat,
                    };
        
                    let token = encode(
                        &Header::default(),
                        &claims,
                        &EncodingKey::from_secret(config.jwt.secret_key.as_ref()),
                    )
                    .unwrap();
        
                    info!("User {:?} logged in. \n    Made token: {:?}", user, token);
        
                    let cookie = Cookie::build("token", token.to_owned())
                        .path("/")
                        .same_site(actix_web::cookie::SameSite::None) // Might need to change later for production, we will see
                        .max_age(ActixWebDuration::new(config.jwt.expiration * 60 * 60 * 24, 0))
                        .http_only(true) // This ensures the cookie is safe to use (can't be accessed through javascript)
                        .finish();
        
                    info!("Made Cookie: {:?}", cookie);
                    
                    HttpResponse::Ok()
                        .cookie(cookie)
                        .json(UserLoginResponse { auth_token: token })
                },
                LoginResponse::UnknownUsernameOrPassword => {
                    info!("Unknown username or password");
                    HttpResponse::BadRequest().json(LoginError::UnknownUsernameOrPassword)
                },
            }
        },
        Err(e) => handle_server_error(e, generic_conflict_handler),
    }
}

#[get("/auth/logout")]
async fn logout_handler(_: jwt_auth::JwtMiddleware) -> impl Responder {
    let cookie = Cookie::build("token", "")
        .path("/")
        .same_site(actix_web::cookie::SameSite::None) // Might need to change later for production, we will see
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success"}))
}

#[get("/user")]
async fn get_me_handler(
    req: HttpRequest,
    db: web::Data<Database>,
    config: web::Data<Config>,
    _: jwt_auth::JwtMiddleware,
) -> impl Responder {
    let ext = req.extensions();
    if let Some(user_id) = ext.get::<uuid::Uuid>() {
        match db.user_db.get_private_data(*user_id, &config) {
            Ok(response) => {
                if let Some(user_data) = response { 
                    HttpResponse::Ok().json(user_data)
                } else {
                    HttpResponse::InternalServerError().json(UserDataError::UserIdNotFound(*user_id))
                }
            },
            Err(e) => handle_server_error(e, generic_conflict_handler),
        }
    } else {
        HttpResponse::Unauthorized().into()
    }
}

#[post("/user/update")]
async fn user_update_handler(
    req: HttpRequest,
    body: web::Json<UserUpdateSchema>,
    db: web::Data<Database>,
    config: web::Data<Config>,
    _: jwt_auth::JwtMiddleware,
) -> impl Responder {
    let ext = req.extensions();
    if let Some(user) = ext.get::<uuid::Uuid>() {
        let user = *user;
        let res = match body.into_inner() {
            UserUpdateSchema::Email(new_email) => db.user_db.update_email(user, new_email),
            UserUpdateSchema::Password(new_password) => db.user_db.update_password(user, new_password),
            UserUpdateSchema::ProfileName(profile_name) => db.user_db.update_profile_name(user, profile_name),
            UserUpdateSchema::ProfileText(profile_text) => db.user_db.update_profile_text(user, profile_text),
            UserUpdateSchema::ProfileCatchphrase(profile_catchphrase) => db.user_db.update_profile_catchphrase(user, profile_catchphrase),
            UserUpdateSchema::ProfilePicture(profile_photo) => {
                match profile_photo.sanatize() {
                    Ok(url) => db.user_db.update_profile_photo(user, url),
                    Err(e) => Err(e),
                }
            },
            UserUpdateSchema::ProfileBanner(profile_banner) => {
                match profile_banner.sanatize() {
                    Ok(url) => db.user_db.update_profile_banner(user, url),
                    Err(e) => Err(e),
                }
            },
            UserUpdateSchema::FavoritedRuleset(id) => todo!(),
            UserUpdateSchema::FavoritedSetting(id) => todo!(),
        };
        match res {
            Ok(response) => {
                match response {
                    crate::database::UpdateResponse::Success => HttpResponse::Ok().into(),
                    crate::database::UpdateResponse::NotFound => HttpResponse::NotFound().json(UserDataError::UserIdNotFound(user)),
                }
            },
            Err(e) => {
                handle_server_error(e, generic_conflict_handler)
            }
        }
    } else {
        HttpResponse::Unauthorized().into()
    }
}


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

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "200MB")]
    file: TempFile,
    name: form::text::Text<String>,
}

#[post("/user/upload")]
async fn user_upload_file(
    req: HttpRequest,
    MultipartForm(form): MultipartForm<UploadForm>,
    db: web::Data<Database>,
    config: web::Data<Config>,
    _: jwt_auth::JwtMiddleware,
) -> impl Responder {
    
    info!("Recieved request to upload {:?}\nwith json: {:?}", form.file, form.name);
    let ext = req.extensions();
    let user_id = ext.get::<uuid::Uuid>().unwrap();
    match db.user_db.get_private_data(*user_id, &config) {
        Ok(response) => {
            if let Some(user_data) = response {
                if user_data.storage_used + form.file.size as i64 <= user_data.storage_limit {
                    let ext = get_file_ext(form.file.file_name);
                    let filepath = &format!(
                        "{}/{}/{}", 
                        config.database.uploads_path,
                        user_data.id, // This is b/c username doesn't change
                        format!("{}.{}", sanitize_filename(&form.name), ext)
                    );
                    let path: &Path = Path::new(filepath);
                    if path.exists() {
                        return HttpResponse::Conflict().json(UploadError::NameConflict(sanitize_filename(&form.name)));
                    }
                    if !is_allowed_file_type(path.extension()) {
                        info!("Invalid extension: {:?} for path: {:?}", path.extension(), path);
                        return HttpResponse::BadRequest().json(UploadError::UnsupportedFileType);
                    }
                    // TODO: 
                    // [x]. Update the storage of the user
                    // [ ]. Update database meta-info on stored files
                    // [x]. Reject file types not supported
                    //    - Currently, only need to support images, such as jepg, png, gif, ico, svg, etc.
                    info!("Attempting to persist: {:?}", path);
                    match form.file.file.persist(filepath) {
                        Ok(_) => {
                            match db.user_db.update_storage_usage(*user_id, user_data.storage_used + form.file.size as i64) {
                                Ok(_) => HttpResponse::Ok().into(),
                                Err(e) => handle_server_error(e, generic_conflict_handler),
                            }
                        },
                        Err(e) => HttpResponse::InternalServerError().json(ServerErrorResponse {
                            error: "Filesystem Error: Persist Error".to_string(),
                            message: e.to_string()
                        }),
                    }
                } else {
                    HttpResponse::InsufficientStorage().json(UploadError::InsufficientUserStorage(user_data.storage_limit - user_data.storage_used, user_data.storage_limit))
                }
            } else {
                HttpResponse::NotFound().json(UploadError::UserNotFound(*user_id))
            }
        },
        Err(e) => handle_server_error(e, generic_conflict_handler),
    }
}

#[get("/user/uploads")]
async fn fetch_user_uploads(
    req: HttpRequest,
    db: web::Data<Database>,
    config: web::Data<Config>,
    _: jwt_auth::JwtMiddleware,
) -> impl Responder {
    let ext = req.extensions();
    let user_id = ext.get::<uuid::Uuid>().unwrap();
    info!("Recieved request to for uploads from user: {}", user_id);
    match db.user_db.get_private_data(*user_id, &config) {
        Ok(response) => {
            if let Some(user_data) = response {
                let filepath = &format!(
                    "{}/{}", 
                    config.database.uploads_path,
                    user_data.id,
                );
                if let Ok(paths) = std::fs::read_dir(filepath) {
                    let mut res = vec![];
                    for path in paths {
                        if let Ok(dir) = path {
                            if let Some(img) = image_data_from_path(dir.path().as_path(), &config) {
                                res.push(img);
                            }
                        }
                    }
                    return HttpResponse::Ok().json(res)
                }
                HttpResponse::InternalServerError().json(
                    ServerErrorResponse {
                        error: "Filesystem Err".to_string(),
                        message: "failed to read internal file".to_string()
                    }
                )

            } else {
                HttpResponse::NotFound().json(UploadError::UserNotFound(*user_id))
            }
        },
        Err(e) => handle_server_error(e, generic_conflict_handler),
    }
}

#[get("/user/uploads/{file_name}")]
async fn fetch_user_upload(
    req: HttpRequest,
    path: web::Path<String>,
    config: web::Data<Config>,
    _: jwt_auth::JwtMiddleware,
) -> impl Responder {
    let ext = req.extensions();
    let user_id = ext.get::<uuid::Uuid>().unwrap();
    let file_name = path.into_inner();
    let filepath = &format!(
        "{}/{}/{}", 
        config.database.uploads_path,
        user_id,
        file_name
    );
    if let Some(img) = image_data_from_path(Path::new(filepath), &config) {
        return HttpResponse::Ok().json(img)
    } else {
        HttpResponse::InternalServerError().json(
            ServerErrorResponse {
                error: "Filesystem Err".to_string(),
                message: "Could not find file".to_string()
            }
        )
    }
}

///////////////////////////////////////////////////
////////////// Helper Functions ///////////////////
///////////////////////////////////////////////////
fn image_data_from_path(p: &Path, config: &Config) -> Option<ImageData> {
    if is_allowed_file_type(p.extension()) {
        if let Ok(file) = File::open(p) {
            return Some(ImageData { 
                src: serverpath_from_filepath(p.to_str().unwrap(), config), 
                name: p.file_name().and_then(|f| f.to_str().and_then(|f| Some(f.to_string()))).unwrap_or_default(), 
                is_external: false, 
                dimen: (0, 0), 
                size: file.metadata().unwrap().len() as i64
            })
        }
    }
    None
} 

fn sanitize_filename(name: &str) -> String {
    name.chars().filter(|c| *c != '/' && *c != '\\').collect()
}

fn generic_conflict_handler<T>(_: T) -> HttpResponse {
    HttpResponse::InternalServerError().json(
        ServerErrorResponse {
            error: "Conflict".to_string(),
            message: "Some internal database conflict occurred.".to_string()
        }
    )
}

fn handle_server_error<T, F>(e: Error<T>, conflict_handler: F) -> HttpResponse 
where 
    F: FnOnce(T) -> HttpResponse,
    T: Debug
{
    warn!("Got server error of type: {:?}", e);
    match e {
        Error::DbConflict(c) => {
            conflict_handler(c)
        },
        Error::DbErr(e) => HttpResponse::InternalServerError().json(
            ServerErrorResponse {
                error: "Database Error".to_string(),
                message: format!("{}", e.to_string())
            }
        ),
        Error::ParseErr(e) => HttpResponse::InternalServerError().json(
            ServerErrorResponse {
                error: "Parse Data Error".to_string(),
                message: format!("{}", e.to_string())
            }
        ),
        Error::Bcrypt(e) => HttpResponse::InternalServerError().json(
            ServerErrorResponse {
                error: "Decryption Error".to_string(),
                message: format!("{}", e.to_string())
            }
        ),
        Error::Other(message) => HttpResponse::InternalServerError().json(
            ServerErrorResponse {
                error: "Other Error".to_string(),
                message,
            }
        ),
    }
}