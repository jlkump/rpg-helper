mod syntax;
mod data;
mod error;
mod network;
mod gui;
mod client;

pub fn run() {
    client::run_app();
}