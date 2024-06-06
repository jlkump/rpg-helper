use anyhow::Result;
use serde::Deserialize;
use std::fs::read_to_string;

#[derive(Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

#[derive(Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16
}

#[derive(Clone, Deserialize)]
pub struct DatabaseConfig {
    pub root_path: String,
}

impl Config {
    pub fn from_file(filename: &str) -> Result<Self> {
        Ok(toml::from_str(&read_to_string(filename)?)?)
    }
}