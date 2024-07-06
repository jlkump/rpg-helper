use std::fmt::Debug;

use actix_web::{cookie::{time::Duration as ActixWebDuration, Cookie}, get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use log::{info, warn};
use serde_json::json;

use crate::{api::jwt_auth::{self, TokenClaims}, model::schema::{UserLoginSchema, UserRegistrationSchema, UserUpdateSchema}, model::types::{ InternalError, ServerError, ServerErrorType, UserLoginResponse }, config::Config, database::User, Database};

use actix_multipart::form::{self, tempfile::TempFile, MultipartForm};

pub fn setup_routes(cfg: &mut web::ServiceConfig) -> &mut web::ServiceConfig {
    let scope = web::scope("/api")
        .service(public_user_handler)
        .service(register_handler)
        .service(login_handler)
        .service(logout_handler)
        .service(get_me_handler)
        .service(user_update_handler)
        .service(user_upload_handler)
        .service(fetch_user_uploads_handler)
        .service(fetch_user_upload_handler);

    cfg.service(scope)
}

#[get("/public/user/{id}")]
async fn public_user_handler(
    path: web::Path<User>,
    db: web::Data<Database>,
) -> impl Responder {
    match db.user_data().get_public_data(path.into_inner()) {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => e.to_http_response(),
    }
}

#[post("/auth/register")]
async fn register_handler(
    body: web::Json<UserRegistrationSchema>, 
    db: web::Data<Database>,
) -> impl Responder {
    match db.user_data().register_user(body.into_inner()) {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => e.to_http_response(),
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
    // TODO: Introduce new middleware to rate-limit the login requests
    match db.user_data().login_user(login.into_inner()) {
        Ok(user) => {
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
        Err(e) => e.to_http_response(),
    }
}

#[get("/auth/logout")]
async fn logout_handler(_: jwt_auth::JwtMiddleware) -> impl Responder {
    let cookie = Cookie::build("token", "")
        .path("/")
        .same_site(actix_web::cookie::SameSite::None) // This must match the cookie setup in login_handler
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
    _: jwt_auth::JwtMiddleware, // TODO: Try to get the user_id from this instead? Should work
) -> impl Responder {
    get_user_from_header(req, |user| {
        match db.user_data().get_private_data(*user) {
            Ok(data) => HttpResponse::Ok().json(data),
            Err(e) => e.to_http_response(),
        }
    })
}

#[post("/user/update")]
async fn user_update_handler(
    req: HttpRequest,
    body: web::Json<UserUpdateSchema>,
    db: web::Data<Database>,
    _: jwt_auth::JwtMiddleware,
) -> impl Responder {
    get_user_from_header(req, |user| {
        match db.user_data().update_user_data(*user, body.into_inner()) {
            Ok(_) => HttpResponse::Ok().into(),
            Err(e) => e.to_http_response(),
        }
    })
}

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(limit = "200MB")]
    file: TempFile,
    name: form::text::Text<String>,
}

#[post("/user/upload")]
async fn user_upload_handler(
    req: HttpRequest,
    MultipartForm(form): MultipartForm<UploadForm>,
    db: web::Data<Database>,
    _: jwt_auth::JwtMiddleware,
) -> impl Responder {
    info!("Recieved request to upload {:?}\nwith json: {:?}", form.file, form.name);
    get_user_from_header(req, |user| {
        match db.user_data().upload(*user, form.file, form.name.0) {
            Ok(_) => HttpResponse::Ok().into(),
            Err(e) => e.to_http_response(),
        }
    })
}

#[get("/user/uploads")]
async fn fetch_user_uploads_handler(
    req: HttpRequest,
    db: web::Data<Database>,
    _: jwt_auth::JwtMiddleware,
) -> impl Responder {
    get_user_from_header(req, |user|{
        info!("Recieved request to for uploads from user: {}", user);
        match db.user_data().get_uploads(*user) {
            Ok(data) => HttpResponse::Ok().json(data),
            Err(e) => e.to_http_response(),
        }
    })
}

#[get("/user/uploads/{file_name}")]
async fn fetch_user_upload_handler(
    req: HttpRequest,
    db: web::Data<Database>,
    path: web::Path<String>,
    _: jwt_auth::JwtMiddleware,
) -> impl Responder {
    get_user_from_header(req, |user| {
        let file_name = path.into_inner();
        info!("Recieved request to for upload {} from user: {}", file_name, user);
        match db.user_data().get_upload(*user, file_name) {
            Ok(data) => HttpResponse::Ok().json(data),
            Err(e) => e.to_http_response(),
        }
    })
}

///////////////////////////////////////////////////
////////////// Helper Functions ///////////////////
///////////////////////////////////////////////////
fn get_user_from_header<F>(req: HttpRequest, f: F) -> HttpResponse
where
    F: FnOnce(&User) -> HttpResponse
{
    let ext = req.extensions();
    let id = ext.get::<User>();
    match id {
        Some(id) => f(id),
        None => {
            // Could be an authentication problem, though unlikely considering we passed the middleware
            warn!("Could not parse ID from HttpRequest");
            ServerError::new(
                ServerErrorType::InternalError(InternalError::Parse), 
                "Could not parse ID from HttpRequest".to_string()
            ).to_http_response()
        },
    }
}