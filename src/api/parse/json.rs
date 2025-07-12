use serde_json::Value;

use crate::api::{parse::ParseError, ApiError};

pub mod data;
pub mod rpg;

pub trait ParseJson
{
    fn from_json(json: Value) -> Result<Self, ApiError> where Self: Sized;
    fn to_json(&self) -> Value;
}

pub enum JsonParseError
{
    InvalidRootValue(Value),
    ExpectedValueNotFound(String),
}

impl From<JsonParseError> for ParseError
{
    fn from(value: JsonParseError) -> Self
    {
        ParseError::JsonErr(value)
    }
}

impl From<JsonParseError> for ApiError
{
    fn from(value: JsonParseError) -> Self
    {
        ApiError::ParseErr(value.into())
    }
}