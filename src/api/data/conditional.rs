use crate::api::data::{tag::Tag, Context};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Conditional
{

}

impl Conditional
{
    pub fn is_true(&self, _: &Context) -> bool
    {
        true
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct ConditionalSet
{

}

impl ConditionalSet
{
    pub fn new() -> ConditionalSet
    {
        todo!()
    }

    pub fn has_conditional(&self, conditional_name: &Tag) -> bool
    {
        todo!()
    }

    pub fn eval(&self, conditional_name: &Tag, context: &Context) -> bool
    {
        todo!()
    }
}