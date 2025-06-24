use crate::api::dataset::tag::Tag;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, PartialOrd, Serialize, Clone)]
pub struct Attribute
{
    base_value: f32,
    current_value: f32,
}

impl Attribute
{
    pub fn new() -> Attribute
    {
        Attribute { base_value: 0.0, current_value: 0.0 }
    }

    pub fn set_base_value(&mut self, base_value: f32)
    {
        self.base_value = base_value;
    }

    pub fn get_base_value(&self) -> f32
    {
        self.base_value
    }

    pub fn set_current_value(&mut self, current_value: f32)
    {
        self.current_value = current_value;
    }

    pub fn get_current_value(&self) -> f32
    {
        self.current_value
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct AttributeSet
{
    attributes: HashMap<Tag, Attribute>
}

impl AttributeSet
{
    pub fn new() -> AttributeSet
    {
        AttributeSet { attributes: HashMap::new() }
    }

    pub fn get(&self, t : &Tag) -> Attribute
    {
        if let Some(a) = self.attributes.get(t)
        {
            a.clone()
        }
        else
        {
            // Default to zero if attribute does not exist
            Attribute::new()
        }
    }

    pub fn get_mut(&mut self, t: &Tag) -> &mut Attribute
    {
        if !self.attributes.contains_key(t)
        {
            self.attributes.insert(t.clone(), Attribute::new());
        }
        self.attributes.get_mut(t).unwrap() // This should never fail. Panic is appropriate if it does
    }
}