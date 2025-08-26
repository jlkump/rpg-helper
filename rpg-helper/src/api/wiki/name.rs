use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::api::data::tag::Tag;

/// A name is the most basic wiki abstraction.
/// It maps a tag to a display name, a string value.
#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct NameRegistry
{
    names: HashMap<Tag, String>,
}

impl NameRegistry
{
    pub fn new() -> NameRegistry
    {
        NameRegistry { names: HashMap::new() }
    }

    /// Defines how a tag should be displayed in a wiki note or page,
    /// if referenced.
    pub fn define_name(&mut self, target: &Tag, display_name: &str)
    {
        self.names.insert(target.clone(), display_name.to_string());
    }

    /// Returns the display name in Ok,
    /// Returns the tag display name in Err in the case the given tag has no defined display name
    pub fn get_display_name<'a, 'b>(&'a self, t: &'b Tag) -> Result<&'a str, &'b str>
    {
        if let Some(name) = self.names.get(t)
        {
            Ok(name)
        }
        else
        {
            Err(t.to_str())
        }
    }
}