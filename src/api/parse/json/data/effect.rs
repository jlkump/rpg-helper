use crate::api::{data::{attribute::Attribute, conditional::Conditional, effect::Effect, equation::Equation, error::DataError, modifier::Modifier, tag::Tag}, parse::json::{JsonParseError, ParseJson}};

use serde_json::{Map, Value};

impl ParseJson for Effect
{
    fn from_json(json: Value) -> Result<Self, crate::api::ApiError> where Self: Sized
    {
        match json
        {
            Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) | Value::Array(_) => Err(JsonParseError::InvalidRootValue(json).into()),
            Value::Object(map) =>
            {
                if let Some((effect_type, effect)) = map.into_iter().next()
                {
                    match effect_type.as_str()
                    {
                        "add-tag" => Ok(Effect::AddStateTag(Tag::from_json(effect)?)),
                        "remove-tag" => Ok(Effect::RemoveStateTag(Tag::from_json(effect)?)),
                        "set-attribute" =>
                        {
                            let atr = Attribute::from_json(effect)?;
                            Ok(Effect::SetAttribute(atr.get_name().clone(), atr.get_value()))
                        },
                        "set-equation" => Ok(Effect::SetEquation(Equation::from_json(effect)?)),
                        "set-conditional" => Ok(Effect::SetConditional(Conditional::from_json(effect)?)),
                        "set-modifier" => Ok(Effect::SetModifier(Modifier::from_json(effect)?)),
                        "set-text" =>
                        {
                            if let Some(m) = effect.as_object()
                            {
                                if let Some((tag, text)) = m.iter().next()
                                {
                                    if let Some(text) = text.as_str()
                                    {
                                        Ok(Effect::SetTextData(Tag::from_str(tag).map_err(|e| Into::<DataError>::into(e))?, text.to_string()))
                                    }
                                    else
                                    {
                                        Err(JsonParseError::InvalidValueFound(text.clone()).into())
                                    }
                                }
                                else
                                {
                                    Err(JsonParseError::ExpectedValueNotFound("text-tag".to_string()).into())
                                }
                            }
                            else
                            {
                                Err(JsonParseError::InvalidValueFound(effect).into())
                            }
                        },
                        _ => Err(JsonParseError::InvalidValueFound(Value::String(effect_type)).into()),
                    }
                }
                else
                {
                    Err(JsonParseError::ExpectedValueNotFound("effect-type".to_string()).into())
                }
            },
        }
    }

    fn to_json(&self) -> Value
    {
        let mut result = Map::new();
        match self
        {
            Effect::AddStateTag(tag) => result.insert("add-tag".to_string(), tag.to_json()),
            Effect::RemoveStateTag(tag) => result.insert("remove-tag".to_string(), tag.to_json()),
            Effect::SetAttribute(t, a) => result.insert("set-attribute".to_string(), Attribute::new(t.clone(), *a).to_json()),
            Effect::SetEquation(equation) => result.insert("set-equation".to_string(), equation.to_json()),
            Effect::SetConditional(conditional) => result.insert("set-conditional".to_string(), conditional.to_json()),
            Effect::SetModifier(modifier) => result.insert("set-modifier".to_string(), modifier.to_json()),
            Effect::SetTextData(tag, text) => 
            {
                let mut data = Map::new();
                data.insert(tag.to_string(), Value::String(text.clone()));
                result.insert("set-text".to_string(), Value::Object(data))
            },
        };
        Value::Object(result)
    }
}

#[cfg(test)]
mod unit_tests 
{
    use crate::api::data::{modifier::ModifierChange, tag::Tag};

    use super::*;

    #[test]
    fn to_from_effect_0()
    {
        let e = Effect::AddStateTag(Tag::from_str("test.add").unwrap());
        let json = Effect::from_json(e.to_json()).unwrap();
        assert_eq!(e, json);
    }

    #[test]
    fn to_from_effect_1()
    {
        let e = Effect::AddStateTag(Tag::from_str("test space add.add").unwrap());
        let json = Effect::from_json(e.to_json()).unwrap();
        assert_eq!(e, json);
    }

    #[test]
    fn to_from_effect_2()
    {
        let e = Effect::RemoveStateTag(Tag::from_str("test space remove.unique").unwrap());
        let json = Effect::from_json(e.to_json()).unwrap();
        assert_eq!(e, json);
    }

    #[test]
    fn to_from_effect_3()
    {
        let e = Effect::SetAttribute(Tag::from_str("change the.atr").unwrap(), 14.012);
        let json = Effect::from_json(e.to_json()).unwrap();
        assert_eq!(e, json);
    }

    #[test]
    fn to_from_effect_4()
    {
        let e = Effect::SetConditional(Conditional::new(Tag::from_str("simple test.cond").unwrap(), "lhs <= rhs").unwrap());
        let json = Effect::from_json(e.to_json()).unwrap();
        assert_eq!(e, json);
    }

    #[test]
    fn to_from_effect_5()
    {
        let e = Effect::SetEquation(Equation::new(Tag::from_str("simple test.eq").unwrap(), "lhs + rhs").unwrap());
        let json = Effect::from_json(e.to_json()).unwrap();
        assert_eq!(e, json);
    }

    #[test]
    fn to_from_effect_6()
    {
        let e = Effect::SetModifier(Modifier { name: Tag::from_str("simple modifier.name").unwrap(), target: Tag::from_str("simple modifier.target").unwrap(), condition: Tag::from_str("simple modifier.cond").unwrap(), change: ModifierChange::BasicValue(1023.3) });
        let json = Effect::from_json(e.to_json()).unwrap();
        assert_eq!(e, json);
    }

    #[test]
    fn to_from_effect_7()
    {
        let e = Effect::SetTextData(Tag::from_str("simple text.tag").unwrap(), "Text data test".to_string());
        let json = Effect::from_json(e.to_json()).unwrap();
        assert_eq!(e, json);
    }
}