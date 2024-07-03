use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;

use crate::database::Error;

use super::types::{ServerError, ServerErrorType};

mod user;

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Rpg-helper website made in Rust.";

    HttpResponse::Ok().json(json!({"status": "success", "message": MESSAGE}))
}

impl ServerError {
    pub fn new(error: ServerErrorType, message: String) -> Self {
        Self {
            error,
            message,
        }
    }

    pub fn to_http_response(self) -> HttpResponse {
        match &self.error {
            ServerErrorType::Authorization(_) => HttpResponse::Unauthorized().json(self),
            ServerErrorType::NotFound(_) => HttpResponse::NotFound().json(self),
            ServerErrorType::InsufficientStorage(_) => HttpResponse::InsufficientStorage().json(self),
            ServerErrorType::FileTooLarge(_) => HttpResponse::InternalServerError().json(self),
            ServerErrorType::Conflict(_) => HttpResponse::Conflict().json(self),
            ServerErrorType::Unsupported(_) => HttpResponse::BadRequest().json(self),
            ServerErrorType::InternalError(_) => HttpResponse::InternalServerError().json(self),
            ServerErrorType::NotImplemented => HttpResponse::NotImplemented().json(self),
        }
    }
}

impl Error {
    pub fn to_http_response(self) -> HttpResponse {
        ServerError::to_http_response(self.into())
    }
}

pub fn setup_routes(cfg: &mut web::ServiceConfig) -> &mut web::ServiceConfig {
    user::setup_routes(cfg);

    cfg.service(health_checker_handler)
}