use crate::api::{data::{error::DataError, tag::{Tag, TagSet}}, parse::json::{JsonParseError, ParseJson}};

use serde_json::{Map, Value};

impl ParseJson for Tag
{
    fn from_json(json: Value) -> Result<Self, crate::api::ApiError> where Self: Sized
    {
        match json
        {
            Value::Null | Value::Bool(_) | Value::Number(_) | Value::Array(_) | Value::Object(_) => Err(JsonParseError::InvalidRootValue(json).into()),
            Value::String(s) =>
            {
                Ok(Tag::from_str(&s).map_err(|e| Into::<DataError>::into(e))?)
            },
        }
    }

    fn to_json(&self) -> Value
    {
        Value::String(self.to_str().to_string())
    }
}

impl ParseJson for TagSet
{
    fn from_json(json: Value) -> Result<Self, crate::api::ApiError> where Self: Sized
    {
        match json
        {
            Value::Object(tags) =>
            {
                let mut set = TagSet::new();
                for (tag, count) in tags
                {
                    let tag = Tag::from_str(&tag).map_err(|e| Into::<DataError>::into(e))?;
                    if let Some(Some(count)) = count.as_number().map(|c| c.as_i64())
                    {
                        for _ in 0..count
                        {
                            set.add_tag(&tag);
                        }
                    }
                }
                Ok(set)
            },
            _ => Err(JsonParseError::InvalidRootValue(json).into())
        }
    }

    fn to_json(&self) -> Value
    {
        let mut m = Map::new();
        for (t, c) in self.iter_primary_tags()
        {
            m.insert(t.to_string(), Value::Number((*c).into()));
        }
        Value::Object(m)
    }
}


#[cfg(test)]
mod unit_tests 
{
    use super::*;


    #[test]
    fn to_from_tag_0()
    {
        let t = Tag::from_str("simple").unwrap();
        let json = Tag::from_json(t.to_json()).unwrap();
        assert_eq!(t, json);
    }

    #[test]
    fn to_from_tag_1()
    {
        let t = Tag::from_str("compound.tag").unwrap();
        let json = Tag::from_json(t.to_json()).unwrap();
        assert_eq!(t, json);
    }

    #[test]
    fn to_from_tag_2()
    {
        let t = Tag::from_str("compound space.tag").unwrap();
        let json = Tag::from_json(t.to_json()).unwrap();
        assert_eq!(t, json);
    }
}