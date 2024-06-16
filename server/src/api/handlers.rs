use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;

mod user;
mod user_files;

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Rpg-helper website made in Rust.";

    HttpResponse::Ok().json(json!({"status": "success", "message": MESSAGE}))
}

pub fn setup_routes(cfg: &mut web::ServiceConfig) -> &mut web::ServiceConfig {
    user::setup_routes(cfg);
    
    let scope = web::scope("/api")
        .service(health_checker_handler);

    cfg.service(scope)
}