mod api;
mod client;
mod data;
mod error;
mod gui;
mod syntax;

extern crate dotenv;

use dotenv::dotenv;

pub fn run() {
    dotenv().ok();
    client::run_app();
}