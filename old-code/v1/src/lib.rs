mod api;
mod client;
mod error;
mod gui;
mod model;
mod router;
mod store;

pub fn run() {
    dotenv::dotenv().ok();
    client::run_app();
}