use crate::api::{data, parse::json::JsonParseError, ApiError};

/// This module handles parsing all data from a given input file type into the associated
/// type in rust and vice versa.
/// 
/// For now, the only supported file-type will be json, but future file types will be placed in the parse module.
pub mod json;

pub enum ParseError
{
    // DataParseErr(data::error::ParseError),
    JsonErr(JsonParseError),
}

impl From<ParseError> for ApiError
{
    fn from(value: ParseError) -> Self
    {
        ApiError::ParseErr(value)
    }
}