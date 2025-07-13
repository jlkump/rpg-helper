use std::collections::HashMap;

use crate::api::{data::{attribute::AttributeSet, conditional::ConditionalSet, context::{Context, RawContextData}, equation::EquationSet, error::DataError, modifier::ModifierSet, tag::{Tag, TagSet}}, parse::json::{JsonParseError, ParseJson}};

use serde_json::{Map, Value};

impl ParseJson for Context
{
    fn from_json(json: Value) -> Result<Self, crate::api::ApiError> where Self: Sized
    {
        match json
        {
            Value::Object(root) =>
            {
                let mut raw_data = RawContextData
                    {
                        general_tags: TagSet::new(),
                        state_tags: TagSet::new(),
                        atrs: AttributeSet::new(),
                        modifiers: ModifierSet::new(),
                        equations: EquationSet::new(),
                        conditionals: ConditionalSet::new(),
                        text_data: HashMap::new(),
                    };
                for (s, v) in root.into_iter()
                {
                    match s.as_str()
                    {
                        "state_tags" => raw_data.state_tags = TagSet::from_json(v)?,
                        "attributes" => raw_data.atrs = AttributeSet::from_json(v)?,
                        "modifiers" => raw_data.modifiers = ModifierSet::from_json(v)?,
                        "equations" => raw_data.equations = EquationSet::from_json(v)?,
                        "conditions" => raw_data.conditionals = ConditionalSet::from_json(v)?,
                        "text_data" =>
                        {
                            raw_data.text_data = HashMap::new();
                            if let Some(text) = v.as_object()
                            {
                                for (tag, text) in text
                                {
                                    if let Some(text) = text.as_str()
                                    {
                                        raw_data.text_data.insert(Tag::from_str(tag).map_err(|e| Into::<DataError>::into(e))?, text.to_string());
                                    }
                                    else
                                    {
                                        return Err(JsonParseError::InvalidValueFound(text.clone()).into())
                                    }
                                }
                            }
                            else
                            {
                                return Err(JsonParseError::InvalidValueFound(v).into())
                            }
                        },
                        _ => return Err(JsonParseError::InvalidValueFound(v).into()),
                    }
                }
                Ok(Context::from_raw(raw_data)?)
            },
            _ => Err(JsonParseError::InvalidRootValue(json).into()),
        }
    }

    fn to_json(&self) -> Value
    {
        let data = self.as_raw();
        let mut result = Map::new();
        result.insert("state_tags".to_string(), data.state_tags.to_json());
        result.insert("attributes".to_string(), data.atrs.to_json());
        result.insert("modifiers".to_string(), data.modifiers.to_json());
        result.insert("equations".to_string(), data.equations.to_json());
        result.insert("conditions".to_string(), data.conditionals.to_json());
        let mut text = Map::new();
        for (tag, t) in data.text_data.iter()
        {
            text.insert(tag.to_string(), Value::String(t.clone()));
        }
        result.insert("text_data".to_string(), Value::Object(text));

        Value::Object(result)
    }
}

#[cfg(test)]
mod unit_tests 
{
    use crate::api::data::{conditional::Conditional, equation::Equation, modifier::{Modifier, ModifierChange}};

    use super::*;


    #[test]
    fn to_from_ctx_0()
    {
        let mut ctx = Context::new();
        ctx.set_attribute(&Tag::from_str("atr.1").unwrap(), 1212.23).unwrap();
        ctx.set_attribute(&Tag::from_str("atr.2").unwrap(), 15.02).unwrap();
        ctx.set_equation(Equation::new(Tag::from_str("test.eq").unwrap(), "atr.1 + 3").unwrap()).unwrap();
        let json = Context::from_json(ctx.to_json()).unwrap();
        assert_eq!(ctx, json);
    }

    #[test]
    fn to_from_ctx_1()
    {
        let mut ctx = Context::new();
        ctx.set_attribute(&Tag::from_str("atr.1").unwrap(), 1212.23).unwrap();
        ctx.set_attribute(&Tag::from_str("atr.2").unwrap(), 15.02).unwrap();
        ctx.set_equation(Equation::new(Tag::from_str("test.eq").unwrap(), "atr.1 + 3").unwrap()).unwrap();
        ctx.set_modifier(Modifier::new(Tag::from_str("modifier.test").unwrap(), Tag::from_str("atr.1").unwrap(), Tag::from_str("true").unwrap(), ModifierChange::BasicValue(1.0))).unwrap();
        ctx.set_conditional(Conditional::new(Tag::from_str("true").unwrap(), "true").unwrap()).unwrap();
        let json = Context::from_json(ctx.to_json()).unwrap();
        assert_eq!(ctx, json);
        assert_eq!(ctx.get_value(&Tag::from_str("atr.1").unwrap()).unwrap().unwrap(), 1213.23)
    }
}