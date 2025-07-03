use serde::{Deserialize, Serialize};

pub mod attribute;
pub mod conditional;
pub mod context;
pub mod effect;
pub mod error;
pub mod evaltree;
pub mod equation;
pub mod modifier;
pub mod tag;

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum DataType
{
    Tag,
    Attribute,
    Condition,
    Modifier,
    Equation,
    Text,
}