use std::path::Path;

use actix_web::{cookie::{time::Duration as ActixWebDuration, Cookie}, get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use log::info;
use serde_json::json;

use crate::{api::{jwt_auth::{self, TokenClaims}, schema::{FileUploadMetadata, UserLoginSchema, UserRegistrationSchema, UserUpdateSchema}, types::{ LoginError, RegistrationError, UploadError, UserDataError, UserLoginResponse}}, config::Config, database::{user::{LoginResponse, RegistrationResponse, User}, Database}};

use actix_multipart::form::{json::Json as MPJson, tempfile::TempFile, MultipartForm};

pub fn setup_routes(cfg: &mut web::ServiceConfig) -> &mut web::ServiceConfig {
    let scope = web::scope("/api")
        .service(login_handler)
        .service(logout_handler)
        .service(register_handler)
        .service(get_me_handler)
        .service(get_user_handler);

    cfg.service(scope)
}

#[post("/auth/register")]
async fn register_handler(
    body: web::Json<UserRegistrationSchema>, 
    db: web::Data<Database>,
    config: web::Data<Config>,
) -> impl Responder {
    let registration_response = db.user_db.register_user(body.into_inner());
    match registration_response {
        RegistrationResponse::Success(user) => {
            return HttpResponse::Ok().json(db.user_db.get_data(user, &config).unwrap());
        },
        RegistrationResponse::EmailTaken => {
            return HttpResponse::Conflict().json(RegistrationError::EmailTaken);
        },
        RegistrationResponse::UsernameTaken => {
            return HttpResponse::Conflict().json(RegistrationError::UsernameTaken);
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
        LoginResponse::Success(user) => {
            let now = Utc::now();
            let iat = now.timestamp() as usize;
            let exp = (now + Duration::days(config.jwt.expiration)).timestamp() as usize;
            let claims: TokenClaims = TokenClaims {
                user_id: user.id.to_string(),
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
        LoginResponse::UnknownUsername => HttpResponse::BadRequest().json(LoginError::UnknownUsernameOrPassword),
        LoginResponse::WrongPassword => HttpResponse::BadRequest().json(LoginError::UnknownUsernameOrPassword) // TODO: Handle multiple tries
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
    let user_id = ext.get::<uuid::Uuid>().unwrap();
    if let Some(user_data) = db.user_db.get_data(user_id.into(), &config) { 
        HttpResponse::Ok().json(user_data)
    } else {
        HttpResponse::InternalServerError().json(UserDataError::UserIdNotFound(*user_id))
    }
}

#[get("/user/{id}")]
async fn get_user_handler(
    path: web::Path<uuid::Uuid>,
    db: web::Data<Database>,
    config: web::Data<Config>,
) -> impl Responder {
    let user = path.into_inner();

    if let Some(data) = db.user_db.get_public_data(user.into(), &config) {
        HttpResponse::Ok().json(data)
    } else {
        HttpResponse::NotFound().json(UserDataError::UserIdNotFound(user))
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
    let user = ext.get::<uuid::Uuid>().unwrap().into();

    if let Some(_) = db.user_db.get_data(user, &config) {
        match body.into_inner() {
            UserUpdateSchema::Email(new_email) => db.user_db.update_user_email(user, new_email),
            UserUpdateSchema::Password(new_password) => db.user_db.update_user_password(user, new_password),
            UserUpdateSchema::ProfileName(profile_name) => db.user_db.update_user_profile_name(user, profile_name),
            UserUpdateSchema::ProfilePicture(_) => todo!(),
        };
        HttpResponse::Ok().into()
    } else {
        HttpResponse::InternalServerError().json(UserDataError::UserIdNotFound(user.id))
    }
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "200MB")]
    file: TempFile,
    json: MPJson<FileUploadMetadata>,
}

#[post("/user/upload")]
async fn user_upload_file(
    req: HttpRequest,
    MultipartForm(form): MultipartForm<UploadForm>,
    db: web::Data<Database>,
    config: web::Data<Config>,
    _: jwt_auth::JwtMiddleware,
) -> impl Responder {
    let ext = req.extensions();
    let user_id = ext.get::<uuid::Uuid>().unwrap();
    info!("Recieved request to upload {:?}\nwith json: {:?}", form.file, form.json);

    if let Some(user_data) = db.user_db.get_data(user_id.into(), &config) {
        if user_data.storage_used + form.file.size as i64 <= user_data.storage_limit {
            let filepath = &format!(
                "{}/uploads/{}/{}", 
                config.database.uploads_path,
                user_data.username, // This is only ok b/c username doesn't change
                sanitize_filename(&form.json.name)
            );
            if !Path::new(filepath).exists() {
                match form.file.file.persist(filepath) {
                    Ok(_) => HttpResponse::Ok().into(),
                    Err(e) => HttpResponse::InternalServerError().json(UploadError::FileSystemErr(e.to_string())),
                }
            } else {
                HttpResponse::Conflict().json(UploadError::NameConflict(sanitize_filename(&form.json.name)))
            }
        } else {
            HttpResponse::InsufficientStorage().json(UploadError::InsufficientUserStorage(form.file.size as i64, user_data.storage_limit - user_data.storage_used))
        }
    } else {
        HttpResponse::InternalServerError().json(UploadError::UserNotFound(*user_id))
    }
}

fn sanitize_filename(name: &str) -> String {
    name.chars().filter(|c| *c != '/' && *c != '\\').collect()
}