use crate::api::{dataset::tag::Tag};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Effect
{
    grant_tags: Vec<Tag>,
    revoke_tags: Vec<Tag>,
    attribute_effects: Vec<AttributeEffect>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum AttributeEffect
{
    Add(Tag, f32),
    Multiply(Tag, f32),
    Divide(Tag, f32),
    Override(Tag, f32),
    Max(Tag, f32),
    Min(Tag, f32),
}