use core::fmt;
use std::future::{ready, Ready};

use actix_web::error::ErrorUnauthorized;
use actix_web::{dev::Payload, Error as ActixWebError};
use actix_web::{http, web, FromRequest, HttpMessage, HttpRequest};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::config::Config;

use super::types::AuthError;

// Implementation uses https://codevoweb.com/rust-jwt-authentication-with-actix-web/ as reference for authorization

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub user_id: String,    // Logged-in user_id
    pub iat: usize,         // Issued time as DateTime<...>.timestamp
    pub exp: usize,         // Expiration time
}

pub struct JwtMiddleware {
    pub user_id: uuid::Uuid, // This struct holds the data that can be requested from handler threads
}

impl FromRequest for JwtMiddleware {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let data = req.app_data::<web::Data<Config>>().unwrap();

        let token = req
            .cookie("token")
            .map(|c| c.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
            });

        if token.is_none() {
            return ready(Err(ErrorUnauthorized(AuthError::NotLoggedIn)));
        }

        let claims = match decode::<TokenClaims>(
            &token.unwrap(),
            &DecodingKey::from_secret(data.jwt.secret_key.as_ref()),
            &Validation::default(),
        ) {
            Ok(c) => c.claims,
            Err(_) => {
                return ready(Err(ErrorUnauthorized(AuthError::InvalidToken)));
            }
        };

        let user_id = uuid::Uuid::parse_str(&claims.user_id).unwrap();
        req.extensions_mut().insert::<uuid::Uuid>(user_id.to_owned());

        ready(Ok(JwtMiddleware { user_id }))
    }
}