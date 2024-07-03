use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web, App, HttpServer};
use api::routes;
use config::Config;
use database::Database;
use log::info;

mod api;
mod config;
mod database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_file("Config.toml").unwrap();
    let db = web::Data::new(Database::open(config.clone()));
    let config_data = web::Data::new(config.clone());

    info!("Starting server at {}:{}/", config.server.host, config.server.port);

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin() // TEMP: Change when put on actual server
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
            .app_data(db.clone())           // Passing handle to user DB to worker threads
            .app_data(config_data.clone())  // Config data for jwt secret and other assorted info
            .configure(routes::initialize)
            .service(actix_files::Files::new("/uploads", config.database.uploads_path.to_string()))
            .service(actix_files::Files::new("/files", config.database.global_files_path.to_string()).show_files_listing())
    })
    .bind((config.server.host, config.server.port))?
    .run()
    .await
}