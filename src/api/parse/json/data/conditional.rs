use crate::api::{data::{conditional::Conditional, error::DataError, tag::Tag}, parse::json::{JsonParseError, ParseJson}, ApiError};

use serde_json::{Map, Value};

impl ParseJson for Conditional
{
    fn from_json(json: Value) -> Result<Self, ApiError> where Self: Sized
    {
        match json
        {
            Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) | Value::Array(_) => Err(JsonParseError::InvalidRootValue(json).into()),
            Value::Object(map) => 
            {
                if let Some(Some(tag)) = map.get("tag").map(|v| v.as_str())
                {
                    if let Some(Some(conditional)) = map.get("conditional").map(|v| v.as_str())
                    {
                        Ok(Conditional::new(Tag::from_str(tag).map_err(|e| Into::<DataError>::into(e))?, conditional)?)
                    }
                    else
                    {
                        Err(JsonParseError::ExpectedValueNotFound("conditional".to_string()).into())
                    }
                } 
                else
                {
                    Err(JsonParseError::ExpectedValueNotFound("tag".to_string()).into())
                }

            },
        }
    }

    fn to_json(&self) -> Value
    {
        let mut m = Map::new();
        m.insert("tag".to_string(), Value::String(self.name.to_str().to_string()));
        m.insert("conditional".to_string(), Value::String(self.get_conditional_as_string()));
        Value::Object(m)
    }
}