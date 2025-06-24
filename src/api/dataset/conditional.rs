use crate::api::dataset::Dataset;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Conditional
{

}

impl Conditional
{
    pub fn is_true(&self, _: &Dataset) -> bool
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
}