use crate::api::data::tag::Tag;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, PartialOrd, Serialize, Clone)]
pub struct Attribute
{
    name: Tag,
    value: f32,
}

impl Attribute
{
    fn new(name: Tag, value: f32) -> Attribute
    {
        Attribute { name, value }
    }

    pub fn set_value(&mut self, v: f32)
    {
        self.value = v;
    }

    pub fn get_value(&self) -> f32
    {
        self.value
    }

    pub fn get_name(&self) -> &Tag
    {
        &self.name
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

    pub fn get(&self, attribute_name: &Tag) -> Option<&Attribute>
    {
        self.attributes.get(attribute_name)
    }

    pub fn get_mut(&mut self, attribute_name: &Tag) -> Option<&mut Attribute>
    {
        self.attributes.get_mut(attribute_name)
    }

    pub fn has_attribute(&self, attribute_name: &Tag) -> bool
    {
        self.attributes.contains_key(attribute_name)
    }

    pub fn set_attribute(&mut self, attribute_name: &Tag, value: f32) -> Option<Attribute>
    {
        self.attributes.insert(attribute_name.clone(), Attribute::new(attribute_name.clone(), value))
    }

    pub fn remove_attribute(&mut self, attribute_name: &Tag) -> Option<Attribute>
    {
        self.attributes.remove(attribute_name)
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, Tag, Attribute>
    {
        self.attributes.iter()
    }
}