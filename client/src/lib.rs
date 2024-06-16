mod api;
mod client;
mod data;
mod error;
mod gui;
mod router;
mod store;
mod syntax;

pub fn run() {
    dotenv::dotenv().ok();
    client::run_app();
}