use actix_web::web;

pub fn initialize(cfg: &mut web::ServiceConfig) {
    crate::api::handlers::setup_routes(cfg);
}