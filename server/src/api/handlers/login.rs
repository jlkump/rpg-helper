use actix_web::{cookie::{Cookie, time::Duration as ActixWebDuration}, post, web, HttpResponse, Responder};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;

use crate::{api::jwt_auth::TokenClaims, config::Config, database::user::{LoginResponse, UserDB, UserLoginSchema}};

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
                .json(json!({"status": "success", "token": token}))
        }, 
        LoginResponse::UnknownUsername => HttpResponse::BadRequest().json(json!({"status": "fail", "message": "Invalid username"})),
        LoginResponse::WrongPassword => HttpResponse::BadRequest().json(json!({"status": "fail", "message": "Invalid password"})) // TODO: Handle limited tries
    }
}