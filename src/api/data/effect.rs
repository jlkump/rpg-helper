use crate::api::data::tag::Tag;

use serde::{Deserialize, Serialize};

// Adds a (state) tag, removes a (state) tag
// Sets and attribute value
// Sets the value of an equation
// Sets the value of a conditional
#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum Effect
{
    AddStateTag(Tag),
    RemoveStateTag(Tag),
    SetAttribute(Tag, f32),
    SetEquation(Tag, String),
    SetConditional(Tag, String),
}