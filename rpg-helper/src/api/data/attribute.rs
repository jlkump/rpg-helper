use crate::api::data::{tag::{Tag, TagTemplate}, template::Template};

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Attribute
{
    name: Tag,
    value: f32,
}

impl Attribute
{
    pub fn new(name: Tag, value: f32) -> Attribute
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

    // Modifies this attribute set such that all attributes are now
    // prefixed with an input tag. For example, consider the attribute set:
    //         date.year:  1240.0
    //         date.month: 5.0
    //         date.day:   15.0
    // Which is modified with prefix tag "lhs":
    //         lhs.date.year:      1240.0
    //         lhs.date.month:     5.0
    //         lhs.date.day:       15.0
    // This is useful for comparision of dates, items, etc. As a context can be
    // made combining both values for the purposes of evaluation.
    pub fn add_prefix(mut self, prefix: &Tag) -> Self
    {
        let mut new_atr = self.attributes.clone();
        for (_, a) in self.attributes.iter()
        {
            let mut a = a.clone();
            let mut new_tag = prefix.to_string();
            new_tag.push('.');
            new_tag.push_str(a.name.to_str());
            let new_tag = Tag::from_str(&new_tag).unwrap(); // Based on current tag logic, this can't fail if the prefix tag is valid.
            a.name = new_tag.clone();
            new_atr.insert(new_tag, a);
        }
        self.attributes = new_atr;
        self
    }
}

impl IntoIterator for AttributeSet
{
    type Item = (Tag, Attribute);

    type IntoIter = std::collections::hash_map::IntoIter<Tag, Attribute>;

    fn into_iter(self) -> Self::IntoIter
    {
        self.attributes.into_iter()
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct AttributeTemplate
{
    name_template: TagTemplate,
    default_value: f32,
}

impl AttributeTemplate
{
    pub fn new(name_template: TagTemplate, default_value: f32) -> AttributeTemplate
    {
        AttributeTemplate { name_template, default_value }
    }
}

impl Template<Attribute> for AttributeTemplate
{
    fn get_required_inputs(&self) -> std::collections::HashSet<String>
    {
        self.name_template.get_required_inputs()
    }

    fn fill_template_value(&mut self, input_name: &str, input_value: &Tag) -> Option<Attribute>
    {
        if let Some(name) = self.name_template.fill_template_value(input_name, input_value)
        {
            Some(Attribute { name, value: self.default_value })
        }
        else
        {
            None
        }
    }

    fn attempt_complete(&self) -> Result<Attribute, super::error::TemplateError>
    {
        match self.name_template.attempt_complete()
        {
            Ok(t) => Ok(Attribute { name: t, value: self.default_value }),
            Err(e) => Err(e),
        }
    }
}