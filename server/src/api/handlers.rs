use actix_web::web;

mod user;

pub fn setup_routes(cfg: &mut web::ServiceConfig) -> &mut web::ServiceConfig {
    let scope = web::scope("/api")
        .service(user::login_handler)
        .service(user::logout_handler)
        .service(user::register_handler)
        .service(user::get_me_handler);

    cfg.service(scope)
}