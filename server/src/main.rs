use actix_cors::Cors;
use actix_web::{App, HttpServer};
use api::routes;
use config::Config;

mod config;
mod database;
mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_file("Config.toml").unwrap();

    println!("Starting server at http://{}:{}/", config.server.host, config.server.port);
    HttpServer::new(move || {
        let cors = Cors::default()
        .allow_any_origin()
        .allow_any_method()
        .allow_any_header()
        .max_age(3600);

        App::new()
            .wrap(cors)
            .configure(routes::initialize)
    })
    .bind((config.server.host, config.server.port))?
    .run()
    .await
}