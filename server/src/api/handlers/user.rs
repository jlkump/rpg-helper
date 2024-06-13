use actix_web::{cookie::{time::Duration as ActixWebDuration, Cookie}, get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;
use log::trace;

use crate::{api::{jwt_auth::{self, TokenClaims}, schema::{UserLoginSchema, UserRegistrationSchema}, types::{ LoginError, RegistrationError, UserDataError, UserDataResponse, UserLoginResponse}}, config::Config, database::user::{LoginResponse, RegistrationResponse, UserDB}};

#[post("/auth/register")]
async fn register_handler(
    body: web::Json<UserRegistrationSchema>, 
    user_db: web::Data<UserDB>
) -> impl Responder {
    trace!("Recieved registration request with body of {:?}", body);
    let registration_response = user_db.register_user(body.into_inner());
    trace!("Got response of {:?} from Database", registration_response);
    match registration_response {
        RegistrationResponse::Success(user) => {
            return HttpResponse::Ok().json(UserDataResponse { data: user_db.get_data(user).unwrap() });
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
    user_db: web::Data<UserDB>, 
    config: web::Data<Config>
) -> impl Responder {
    // When the user login is successful,
    // the user has secure login. Otherwise, not logged in.
    match user_db.login_user(login.into_inner()) {
        LoginResponse::Success(user) => {
            let now = Utc::now();
            let iat = now.timestamp() as usize;
            let exp = (now + Duration::minutes(config.jwt.expiration)).timestamp() as usize;
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
                .max_age(ActixWebDuration::new(config.jwt.expiration * 60, 0))
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
    user_db: web::Data<UserDB>,
    _: jwt_auth::JwtMiddleware,
) -> impl Responder {
    let ext = req.extensions();
    let user_id = ext.get::<uuid::Uuid>().unwrap();

    if let Some(user_data) = user_db.get_data(user_id.into()) { 
        HttpResponse::Ok().json(UserDataResponse { data: user_data} )
    } else {
        HttpResponse::InternalServerError().json(UserDataError::UserNotFound(*user_id))
    }

}