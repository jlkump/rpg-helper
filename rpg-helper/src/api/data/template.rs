use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::api::data::{attribute::AttributeTemplate, conditional::ConditionalTemplate, equation::EquationTemplate, error::TemplateError, modifier::ModifierTemplate, tag::{Tag, TagTemplate}};

/// Used to indicate that a struct is a template struct.
/// Mainly useful for the Templated enum such that
/// insert templated value can be easily standardized
pub trait Template<T>
    where T: Sized
{
    fn get_required_inputs(&self) -> HashSet<String>;

    fn fill_template_value(&mut self, input_name: &str, input_value: &Tag) -> Option<T>;

    fn attempt_complete(&self) -> Result<T, TemplateError>;

    fn is_complete(&self) -> bool
    {
        self.get_required_inputs().is_empty()
    }
}

/// Encapsulation of a possible value that can be either a template or completed value
/// Implements some helper functionality to allow for easily inserting template
/// values until the template is complete.
#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum Templated<T, C>
where 
    T: Template<C> + Clone, 
    C: Sized + Clone
{
    Template(T),
    Complete(C),
}

impl<T, C> Templated<T, C>
where 
    T: Template<C> + Clone, 
    C: Sized + Clone
{
    /// If this templated value is still a template
    /// this will attempt to insert the given template value
    /// Otherwise, this will be a no-op
    pub fn fill_template_value(&mut self, input_name: &str, input_value: &Tag)
    {
        if let Templated::Template(t) = self
        {
            if let Some(v) = t.fill_template_value(input_name, input_value)
            {
                *self = Templated::Complete(v);
            }
        }
    }

    /// The values required to be filled in inside template tags.
    /// All templates are some combinations of templated tags,
    /// this returns the inputs (the values inside brackets)
    /// of the template's template tags.
    /// 
    /// The value of the string can be used to fill in
    /// the corresponding tag input value by pairing it with
    /// a tag in `insert_template_value`.
    pub fn get_required_inputs(&self) -> HashSet<String>
    {
        match self
        {
            Templated::Template(t) => t.get_required_inputs(),
            Templated::Complete(_) => HashSet::new(),
        }
    }

    pub fn is_complete(&self) -> bool
    {
        match self
        {
            Templated::Template(_) => false,
            Templated::Complete(_) => true,
        }
    }

    /// Convert this templated value into a completed
    /// value (if it exists). None is returned if the
    /// value is not finished templating.
    pub fn into_complete(self) -> Option<C>
    {
        match self
        {
            Templated::Template(_) => None,
            Templated::Complete(c) => Some(c),
        }
    }

    /// Convert this templated value into a completed value reference.
    /// A less destructive version of into_complete
    pub fn as_complete(&self) -> Option<&C>
    {
        match self
        {
            Templated::Template(_) => None,
            Templated::Complete(c) => Some(c),
        }
    }

    pub fn as_complete_mut(&mut self) -> Option<&mut C>
    {
        match self
        {
            Templated::Template(_) => None,
            Templated::Complete(c) => Some(c),
        }
    }

    /// Convert this templated value into just the template
    /// value. Fails if the templated value is already complete.
    pub fn into_template(self) -> Option<T>
    {
        match self
        {
            Templated::Template(t) => Some(t),
            Templated::Complete(_) => None,
        }
    }

    /// Convert this templated value into a template value reference.
    /// A less destructive version of `into_template`.
    pub fn as_template(&self) -> Option<&T>
    {
        match self
        {
            Templated::Template(t) => Some(t),
            Templated::Complete(_) => None,
        }
    }

    pub fn as_template_mut(&mut self) -> Option<&mut T>
    {
        match self
        {
            Templated::Template(t) => Some(t),
            Templated::Complete(_) => None,
        }
    }
}

/// Simple Enum wrapper for all possible template values in this api layer
#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum TemplateValue
{
    Attribute(AttributeTemplate),
    Conditional(ConditionalTemplate),
    Equation(EquationTemplate),
    Modifier(ModifierTemplate),
    Tag(TagTemplate),
}

impl TemplateValue
{
    /// Simply calls get_required_inputs on the inner value
    pub fn get_required_inputs(&self) -> HashSet<String>
    {
        match self
        {
            TemplateValue::Attribute(attribute_template) => attribute_template.get_required_inputs(),
            TemplateValue::Conditional(conditional_template) => conditional_template.get_required_inputs(),
            TemplateValue::Equation(equation_template) => equation_template.get_required_inputs(),
            TemplateValue::Modifier(modifier_template) => modifier_template.get_required_inputs(),
            TemplateValue::Tag(tag_template) => tag_template.get_required_inputs(),
        }
    }

    pub fn is_complete(&self) -> bool
    {
        match self
        {
            TemplateValue::Attribute(attribute_template) => attribute_template.is_complete(),
            TemplateValue::Conditional(conditional_template) => conditional_template.is_complete(),
            TemplateValue::Equation(equation_template) => equation_template.is_complete(),
            TemplateValue::Modifier(modifier_template) => modifier_template.is_complete(),
            TemplateValue::Tag(tag_template) => tag_template.is_complete(),
        }
    }
}