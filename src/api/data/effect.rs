use crate::api::data::{conditional::Conditional, equation::Equation, modifier::Modifier, tag::Tag};

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
    SetEquation(Equation),
    SetConditional(Conditional),
    SetModifier(Modifier),
    SetTextData(Tag, String),
}