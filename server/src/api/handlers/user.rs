use actix_web::{cookie::{time::Duration as ActixWebDuration, Cookie}, get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;

use crate::{api::{jwt_auth::{self, TokenClaims}, schema::{UserLoginSchema, UserRegistrationSchema}, types::{ LoginError, RegistrationError, UserDataError, UserDataResponse, UserLoginResponse}}, config::Config, database::{user::{LoginResponse, RegistrationResponse}, Database}};

pub fn setup_routes(cfg: &mut web::ServiceConfig) -> &mut web::ServiceConfig {
    let scope = web::scope("/api")
        .service(login_handler)
        .service(logout_handler)
        .service(register_handler)
        .service(get_me_handler);

    cfg.service(scope)
}

#[post("/auth/register")]
async fn register_handler(
    body: web::Json<UserRegistrationSchema>, 
    db: web::Data<Database>,
) -> impl Responder {
    let registration_response = db.user_db.register_user(body.into_inner());
    match registration_response {
        RegistrationResponse::Success(user) => {
            return HttpResponse::Ok().json(UserDataResponse { data: db.user_db.get_data(user).unwrap() });
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
            let exp = (now + Duration::minutes(config.jwt.expiration * 60 * 24)).timestamp() as usize;
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

            let cookie = Cookie::build("token", token.to_owned())
                .path("/")
                .max_age(ActixWebDuration::new(config.jwt.expiration * 60 * 60 * 24, 0))
                .http_only(true) // This ensures the cookie is safe to use (can't be accessed through javascript)
                .finish();

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
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success"}))
}

#[get("/user/me")]
async fn get_me_handler(
    req: HttpRequest,
    db: web::Data<Database>,
    _: jwt_auth::JwtMiddleware,
) -> impl Responder {
    let ext = req.extensions();
    let user_id = ext.get::<uuid::Uuid>().unwrap();

    if let Some(user_data) = db.user_db.get_data(user_id.into()) { 
        HttpResponse::Ok().json(UserDataResponse { data: user_data} )
    } else {
        HttpResponse::InternalServerError().json(UserDataError::UserNotFound(*user_id))
    }

}