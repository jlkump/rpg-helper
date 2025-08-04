use crate::api::{data::{equation::{Equation, EquationSet}, error::DataError, tag::Tag}, parse::json::{JsonParseError, ParseJson}};

use serde_json::{Map, Value};

impl ParseJson for Equation
{
    fn from_json(json: Value) -> Result<Self, crate::api::ApiError> where Self: Sized
    {
        if let Some(m) = json.as_object()
        {
            if let Some(Some(name)) = m.get("name").map(|n| n.as_str())
            {
                if let Some(Some(eq)) = m.get("equation").map(|e| e.as_str())
                {
                    Ok(Equation::new(Tag::from_str(name).map_err(|e| Into::<DataError>::into(e))?, eq)?)
                }
                else
                {
                    Err(JsonParseError::ExpectedValueNotFound("equation".to_string()).into())
                }
            }
            else
            {
                Err(JsonParseError::ExpectedValueNotFound("name".to_string()).into())
            }
        }
        else
        {
            Err(JsonParseError::InvalidRootValue(json).into())
        }
    }

    fn to_json(&self) -> Value
    {
        let mut m = Map::new();
        m.insert("name".to_string(), Value::String(self.name.to_string()));
        m.insert("equation".to_string(), Value::String(self.get_equation_string()));
        Value::Object(m)
    }
}

impl ParseJson for EquationSet
{
    fn from_json(json: Value) -> Result<Self, crate::api::ApiError> where Self: Sized
    {
        match json
        {
            Value::Array(equations) =>
            {
                let mut set = EquationSet::new();
                for e in equations
                {
                    set.set_equation(Equation::from_json(e)?);
                }
                Ok(set)
            },
            _ => Err(JsonParseError::InvalidRootValue(json).into()),
        }
    }

    fn to_json(&self) -> Value
    {
        let mut result = vec![];
        for (_, e) in self.iter()
        {
            result.push(e.to_json());
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
    fn to_from_eq_0()
    {
        let e = Equation::new(Tag::from_str("simple").unwrap(), "simple.tag.ref / 5.0").unwrap();
        let json = Equation::from_json(e.to_json()).unwrap();
        assert_eq!(e, json);
    }

    #[test]
    fn to_from_eq_1()
    {
        let e = Equation::new(Tag::from_str("simple").unwrap(), "rounddown((sqrt(8 * Ability.Magic Theory.Exp + 1)-1)/2)").unwrap();
        let json = Equation::from_json(e.to_json()).unwrap();
        assert_eq!(e, json);
    }

    #[test]
    fn to_from_eq_set_0()
    {
        let e = Equation::new(Tag::from_str("simple").unwrap(), "simple.tag.ref / 5.0").unwrap();

        let mut s = EquationSet::new();
        s.set_equation(e);
        let e = Equation::new(Tag::from_str("simple").unwrap(), "rounddown((sqrt(8 * Ability.Magic Theory.Exp + 1)-1)/2)").unwrap();
        s.set_equation(e);

        let json = EquationSet::from_json(s.to_json()).unwrap();
        assert_eq!(s, json);
    }
}