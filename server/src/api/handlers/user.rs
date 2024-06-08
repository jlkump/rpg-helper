use actix_web::{cookie::{time::Duration as ActixWebDuration, Cookie}, get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;

use crate::{api::jwt_auth::{self, TokenClaims}, config::Config, database::user::{LoginResponse, RegistrationResponse, UserDB, UserLoginSchema, UserRegistrationSchema}};

#[post("/auth/register")]
async fn register_handler(
    body: web::Json<UserRegistrationSchema>, 
    db: web::Data<UserDB>
) -> impl Responder {
    let registration_response = db.register_user(body.into_inner());

    match registration_response {
        RegistrationResponse::Success(user_id) => {
            // TODO: Replace with serialized structs instead for more uniform access patterns.
            let user_respose = serde_json::json!({"status": "success", "data": serde_json::json!({"user": user_id})});
            return HttpResponse::Ok().json(user_respose);
        },
        RegistrationResponse::EmailTaken => {
            return HttpResponse::InternalServerError().json(serde_json::json!({"status": "error", "message": "Email Taken"}));
        },
        RegistrationResponse::UsernameTaken => {
            return HttpResponse::InternalServerError().json(serde_json::json!({"status": "error", "message": "Username Taken"}));
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
                .json(json!({"status": "success", "token": token}))
        }, 
        LoginResponse::UnknownUsername => HttpResponse::BadRequest().json(json!({"status": "fail", "message": "Invalid username"})),
        LoginResponse::WrongPassword => HttpResponse::BadRequest().json(json!({"status": "fail", "message": "Invalid password"})) // TODO: Handle limited tries
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

    let user_data = user_db.get_data(user_id.into());

    let json_response = serde_json::json!({
        "status":  "success",
        "data": serde_json::json!({
            "user": user_data
        })
    });

    HttpResponse::Ok().json(json_response)
}