mod syntax;
mod data;
mod error;
mod network;
mod gui;
mod client;

#[macro_use]
extern crate dotenv_codegen;

pub fn run() {
    client::run_app();
}