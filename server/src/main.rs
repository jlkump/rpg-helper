use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web, App, HttpServer};
use api::routes;
use config::Config;
use database::user::UserDB;
use log::info;

mod api;
mod config;
mod database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_file("Config.toml").unwrap();
    let user_db = web::Data::new(UserDB::open(&config));
    let config_data = web::Data::new(config.clone());

    info!("Starting server at {}:{}/ with allowed origin: {}", config.server.host, config.server.port, config.server.origin_path);

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:3000")
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
                header::ORIGIN,
            ])
            .supports_credentials();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(user_db.clone())      // Passing handle to user DB to worker threads
            .app_data(config_data.clone())  // Config data for jwt secret and other assorted info
            .configure(routes::initialize)
    })
    .bind((config.server.host, config.server.port))?
    .run()
    .await
}