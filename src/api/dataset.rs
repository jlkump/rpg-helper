use crate::api::dataset::{attribute::AttributeSet, conditional::ConditionalSet, effect::Effect, modifier::{ModifierSet, ModifierSpec}, tag::{Tag, TagSet}};

use serde::{Deserialize, Serialize};

pub mod attribute;
pub mod conditional;
pub mod effect;
pub mod modifier;
pub mod tag;

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Dataset
{
    atrs: AttributeSet,
    tags: TagSet,
    conditionals: ConditionalSet,
    modifiers: ModifierSet,
}

impl Dataset
{
    pub fn new() -> Dataset
    {
        Dataset { atrs: AttributeSet::new(), tags: TagSet::new(), conditionals: ConditionalSet::new(), modifiers: ModifierSet::new() }
    }

    pub fn has_tag(&self, t: &Tag) -> bool
    {
        self.tags.has_tag(t)
    }

    pub fn get_value(&self, t: &Tag) -> f32
    {
        self.atrs.get(t).get_current_value()
    }

    pub fn apply_effect(&mut self, e: Effect)
    {
        todo!()
    }

    pub fn eval_conditional(&self, t: &Tag) -> bool
    {
        todo!()
    }

    pub fn has_modifier(&self, t: &Tag) -> bool
    {
        todo!()
    }

    pub fn add_modifier(&mut self, spec: ModifierSpec)
    {
        todo!()
    }

    pub fn remove_modifier(&mut self, t: &Tag)
    {
        todo!()
    }
}