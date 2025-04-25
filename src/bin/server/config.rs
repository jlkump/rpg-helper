use anyhow::Result;
use serde::Deserialize;
use std::fs::read_to_string;

#[derive(Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub jwt: JWTConfig,
}

#[derive(Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub origin: String,
}

#[derive(Clone, Deserialize)]
pub struct DatabaseConfig {
    pub database_path: String,
    pub uploads_path: String,
    pub global_files_path: String,
}

#[derive(Clone, Deserialize)]
pub struct JWTConfig {
    pub secret_key: String,
    pub expiration: i64
}

impl Config {
    pub fn from_file(filename: &str) -> Result<Self> {
        Ok(toml::from_str(&read_to_string(filename)?)?)
    }
}