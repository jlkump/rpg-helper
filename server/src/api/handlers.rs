use actix_web::web;

mod login;
mod logout;
mod register;

pub fn setup_routes(cfg: &mut web::ServiceConfig) -> &mut web::ServiceConfig {
    let scope = web::scope("/api")
        .service(login::login_handler)
        .service(logout::logout_handler)
        .service(register::register_handler);

    cfg.service(scope)
}