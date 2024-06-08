use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use api::routes;
use config::Config;
use database::user::UserDB;

mod config;
mod database;
mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_file("Config.toml").unwrap();
    let user_db = web::Data::new(UserDB::open(&config));
    let config_data = web::Data::new(config.clone());

    println!("Starting server at {}:{}/", config.server.host, config.server.port);
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin() // TODO: Change later to allow only certain Origins, Methods, and Headers
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(user_db.clone())      // Passing handle to user DB to worker threads
            .app_data(config_data.clone())  // Config data for jwt secret and other assorted info
            .configure(routes::initialize)
    })
    .bind((config.server.host, config.server.port))?
    .run()
    .await
}

// How to get the UserDB database info
async fn index(data: web::Data<UserDB>) -> String {
    todo!();
    // let mut counter = data.get_data(0);
}