use crate::api::dataset::{conditional::Conditional, effect::Effect, tag::Tag};

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// A modifier conditionally applies an effect
#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Modifier
{
    condition: Conditional,
    effect: Effect,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct ModifierSet
{
    modifiers: HashMap<Tag, Modifier>,
}

impl ModifierSet
{
    pub fn new() -> ModifierSet
    {
        todo!()
    }
}

pub struct ModifierSpec
{
    name: Tag,
    condition: Conditional,
    effect: Effect,
}