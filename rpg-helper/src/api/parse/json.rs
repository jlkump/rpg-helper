use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::{parse::ParseError, ApiError};

pub mod data;
pub mod rpg;

pub trait ParseJson
{
    fn from_json(json: Value) -> Result<Self, ApiError> where Self: Sized;
    fn to_json(&self) -> Value;
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum JsonParseError
{
    InvalidRootValue(Value),
    ExpectedValueNotFound(String),
    DuplicateValueFound(String),
    InvalidValueFound(Value),
    SerdeJsonErr(String),
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

impl From<serde_json::Error> for ParseError
{
    fn from(value: serde_json::Error) -> Self
    {
        JsonParseError::SerdeJsonErr(value.to_string()).into()
    }
}

impl From<serde_json::Error> for ApiError
{
    fn from(value: serde_json::Error) -> Self
    {
        ApiError::ParseErr(value.into())
    }
}