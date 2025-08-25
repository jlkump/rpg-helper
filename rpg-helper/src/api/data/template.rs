use std::collections::HashSet;

use crate::api::data::{error::TemplateError, tag::Tag};

// Template trait may be useful for standardization?
pub trait Template<T>
    where T: Sized
{
    fn get_required_inputs(&self) -> HashSet<String>;

    fn insert_template_value(&mut self, input_name: &str, input_value: &Tag) -> Option<T>;

    fn attempt_complete(&self) -> Result<T, TemplateError>;
}

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
    pub fn insert_template_value(&mut self, input_name: &str, input_value: &Tag)
    {
        if let Templated::Template(t) = self
        {
            if let Some(v) = t.insert_template_value(input_name, input_value)
            {
                *self = Templated::Complete(v);
            }
        }
    }

    pub fn get_required_inputs(&self) -> HashSet<String>
    {
        match self
        {
            Templated::Template(t) => t.get_required_inputs(),
            Templated::Complete(_) => HashSet::new(),
        }
    }
}