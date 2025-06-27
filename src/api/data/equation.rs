use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::api::data::{error::DataError, tag::Tag, Context};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Equation
{
    name: Tag,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct EquationSet
{
    equations: HashMap<Tag, Equation>,

}

impl EquationSet
{
    pub fn new() -> EquationSet
    {
        todo!()
    }

    pub fn eval(&self, t: &Tag, context: &Context) -> Result<f32, DataError>
    {
        todo!()
    }

    pub fn can_eval(&self, t: &Tag) -> bool
    {
        todo!()
    }
}