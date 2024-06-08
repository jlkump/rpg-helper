mod api;
mod client;
mod data;
mod error;
mod gui;
mod syntax;

#[macro_use]
extern crate dotenv_codegen;

pub fn run() {
    client::run_app();
}