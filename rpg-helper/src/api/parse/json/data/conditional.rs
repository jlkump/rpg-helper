use crate::api::{data::{conditional::{Conditional, ConditionalSet}, error::DataError, tag::Tag}, parse::json::{JsonParseError, ParseJson}, ApiError};

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
        m.insert("conditional".to_string(), Value::String(self.get_equation_string()));
        Value::Object(m)
    }
}

impl ParseJson for ConditionalSet
{
    fn from_json(json: Value) -> Result<Self, ApiError> where Self: Sized
    {
        match json
        {
            Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) | Value::Object(_) => Err(JsonParseError::InvalidRootValue(json).into()),
            Value::Array(conditions) =>
            {
                let mut set = ConditionalSet::new();
                for c in conditions
                {
                    set.set_conditional(Conditional::from_json(c)?);
                }
                Ok(set)
            },
        }
    }

    fn to_json(&self) -> Value
    {
        let mut result = vec![];
        for (_, con) in self.iter()
        {
            result.push(con.to_json());
        }
        Value::Array(result)
    }
}

#[cfg(test)]
mod unit_tests 
{
    use crate::api::data::tag::Tag;

    use super::*;

    #[test]
    fn to_from_con_0()
    {
        let c = Conditional::new(Tag::from_str("simple").unwrap(), "true").unwrap();
        let json = Conditional::from_json(c.to_json()).unwrap();
        assert_eq!(c, json);
    }
    
    #[test]
    fn to_from_con_1()
    {
        let c = Conditional::new(Tag::from_str("compound.tag").unwrap(), "true").unwrap();
        let json = Conditional::from_json(c.to_json()).unwrap();
        assert_eq!(c, json);
    }

    #[test]
    fn to_from_con_2()
    {
        let c = Conditional::new(Tag::from_str("compound space.tag").unwrap(), "true").unwrap();
        let json = Conditional::from_json(c.to_json()).unwrap();
        assert_eq!(c, json);
    }

    #[test]
    fn to_from_con_3()
    {
        let c = Conditional::new(Tag::from_str("tag").unwrap(), "lhs > rhs.exp").unwrap();
        let json = Conditional::from_json(c.to_json()).unwrap();
        assert_eq!(c, json);
    }

    #[test]
    fn to_from_con_set_0()
    {
        let c = Conditional::new(Tag::from_str("tag").unwrap(), "lhs > rhs.exp").unwrap();
        let mut s = ConditionalSet::new();
        s.set_conditional(c);
        let c = Conditional::new(Tag::from_str("other").unwrap(), "lhs.other < rhs.exp").unwrap();
        s.set_conditional(c);

        let json = ConditionalSet::from_json(s.to_json()).unwrap();
        assert_eq!(s, json);
    }
}