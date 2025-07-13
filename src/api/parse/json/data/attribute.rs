use crate::api::{data::{attribute::{Attribute, AttributeSet}, tag::Tag}, parse::json::{JsonParseError, ParseJson}};

use serde_json::{Map, Number, Value};

impl ParseJson for Attribute
{
    fn from_json(json: Value) -> Result<Self, crate::api::ApiError> where Self: Sized
    {
        match json
        {
            Value::Object(m) =>
            {
                let mut name = None;
                let mut val = None;
                for (s, v) in m.into_iter()
                {
                    match s.as_str()
                    {
                        "name" => 
                        {
                            if name.is_none()
                            {
                                name = Some(Tag::from_json(v)?);
                            }
                            else
                            {
                                return Err(JsonParseError::DuplicateValueFound(s).into())
                            }
                        },
                        "value" =>
                        {
                            if val.is_none()
                            {
                                if let Some(v) = v.as_f64()
                                {
                                    val = Some(v);
                                }
                                else
                                {
                                    return Err(JsonParseError::InvalidValueFound(v).into())
                                }
                            }
                            else
                            {
                                return Err(JsonParseError::DuplicateValueFound(s).into())
                            }
                        },
                        _ => return Err(JsonParseError::InvalidValueFound(v).into())
                    }
                }
                if let Some(n) = name
                {
                    if let Some(v) = val
                    {
                        Ok(Attribute::new(n, v as f32))
                    }
                    else
                    {
                        Err(JsonParseError::ExpectedValueNotFound("value".to_string()).into())
                    }
                }
                else
                {
                    Err(JsonParseError::ExpectedValueNotFound("name".to_string()).into())
                }
            },
            _  => Err(JsonParseError::InvalidRootValue(json).into()),
        }
    }

    fn to_json(&self) -> Value
    {
        let mut result = Map::new();
        result.insert("name".to_string(), Value::String(self.get_name().to_string()));
        let n = Number::from_f64(self.get_value() as f64).unwrap();
        result.insert("value".to_string(), Value::Number(n));
        Value::Object(result)
    }
}

impl ParseJson for AttributeSet
{
    fn from_json(json: Value) -> Result<Self, crate::api::ApiError> where Self: Sized
    {
        match json
        {
            Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) | Value::Object(_) => Err(JsonParseError::InvalidRootValue(json).into()),
            Value::Array(atrs) =>
            {
                let mut result = AttributeSet::new();
                for a in atrs
                {
                    let a = Attribute::from_json(a)?;
                    result.set_attribute(a.get_name(), a.get_value());
                }
                Ok(result)
            },
        }
    }

    fn to_json(&self) -> Value
    {
        let mut res = vec![];
        for (_, a) in self.iter()
        {
            res.push(a.to_json());
        }
        Value::Array(res)
    }
}

#[cfg(test)]
mod unit_tests 
{
    use crate::api::data::tag::Tag;

    use super::*;

    #[test]
    fn to_from_atr_0()
    {
        let a = Attribute::new(Tag::from_str("simple").unwrap(), 1.02);
        let json = Attribute::from_json(a.to_json()).unwrap();
        assert_eq!(a, json);
    }

    #[test]
    fn to_from_atr_1()
    {
        let a = Attribute::new(Tag::from_str("compound.tag").unwrap(), 11.02);
        let json = Attribute::from_json(a.to_json()).unwrap();
        assert_eq!(a, json);
    }

    #[test]
    fn to_from_atr_2()
    {
        let a = Attribute::new(Tag::from_str("compound space.tag").unwrap(), 13.02);
        let json = Attribute::from_json(a.to_json()).unwrap();
        assert_eq!(a, json);
    }

    #[test]
    fn to_from_atr_set_0()
    {
        let mut s = AttributeSet::new();
        s.set_attribute(&Tag::from_str("compound space.tag").unwrap(), 13.02);
        s.set_attribute(&Tag::from_str("compound.tag").unwrap(), 1.02);

        let json = AttributeSet::from_json(s.to_json()).unwrap();
        assert_eq!(s, json);
    }
}