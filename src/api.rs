use crate::api::{data::error::DataError, parse::ParseError};

pub mod data;
pub mod parse;
pub mod rpg;

pub enum ApiError
{
    DataErr(DataError),
    ParseErr(ParseError),
}