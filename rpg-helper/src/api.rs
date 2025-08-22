use serde::{Deserialize, Serialize};

use crate::api::{data::error::DataError, parse::ParseError};

pub mod data;
pub mod database;
pub mod display;
pub mod parse;
pub mod permission;
pub mod rpg;
pub mod wiki;

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum ApiError
{
    DataErr(DataError),
    ParseErr(ParseError),
}