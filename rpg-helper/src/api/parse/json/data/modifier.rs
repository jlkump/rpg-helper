use serde_json::Value;

use crate::api::{data::modifier::{Modifier, ModifierSet}, parse::json::{JsonParseError, ParseJson}};

impl ParseJson for Modifier
{
    fn from_json(json: Value) -> Result<Self, crate::api::ApiError> where Self: Sized
    {
        Ok(serde_json::from_value(json)?)
    }

    fn to_json(&self) -> Value
    {
        serde_json::to_value(self).unwrap()
    }
}

impl ParseJson for ModifierSet
{
    fn from_json(json: Value) -> Result<Self, crate::api::ApiError> where Self: Sized
    {
        match json
        {
            Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) | Value::Object(_) => Err(JsonParseError::InvalidRootValue(json).into()),
            Value::Array(mods) => 
            {
                let mut set = ModifierSet::new();
                for m in mods
                {
                    set.add_modifier(Modifier::from_json(m)?);
                }
                Ok(set)
            },
        }
    }

    fn to_json(&self) -> Value
    {
        let mut result = vec![];
        for (_, m) in self.iter()
        {
            result.push(m.to_json());
        }
        Value::Array(result)
    }
}

#[cfg(test)]
mod unit_tests 
{
    use crate::api::data::{modifier::ModifierChange, tag::Tag};

    use super::*;


    #[test]
    fn to_from_tag_0()
    {
        let m = Modifier::new(Tag::from_str("Simple modifier.name").unwrap(), Tag::from_str("simple modifier.target").unwrap(), Tag::from_str("Simple modifier.cond").unwrap(), ModifierChange::BasicValue(1024.0));
        let json = Modifier::from_json(m.to_json()).unwrap();
        assert_eq!(m, json);
    }

    #[test]
    fn to_from_tag_1()
    {
        let m = Modifier::new(Tag::from_str("Simple modifier.name").unwrap(), Tag::from_str("simple modifier.target").unwrap(), Tag::from_str("Simple modifier.cond").unwrap(), ModifierChange::FromOtherValue(Tag::from_str("Modifier simple.ref").unwrap()));
        let json = Modifier::from_json(m.to_json()).unwrap();
        assert_eq!(m, json);
    }
}