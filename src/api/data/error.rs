use serde::{Deserialize, Serialize};

use crate::api::data::{evaltree::EvalError, tag::Tag};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum DataError
{
    DoesNotExist(DoesNotExistError),
    ConflictingExpectedType(ConflictError),
    InvalidState(String),
    Evaluation(EvalError),
}

impl DataError
{
    pub fn tag_dne(t: Tag) -> DataError
    {
        DataError::DoesNotExist(DoesNotExistError::Tag(t))
    }

    pub fn attribute_dne(t: Tag) -> DataError
    {
        DataError::DoesNotExist(DoesNotExistError::Attribute(t))
    }

    pub fn condition_dne(t: Tag) -> DataError
    {
        DataError::DoesNotExist(DoesNotExistError::Condition(t))
    }

    pub fn modifier_dne(t: Tag) -> DataError
    {
        DataError::DoesNotExist(DoesNotExistError::Modifier(t))
    }

    pub fn equation_dne(t: Tag) -> DataError
    {
        DataError::DoesNotExist(DoesNotExistError::Equation(t))
    }

    pub fn value_dne(t: Tag) -> DataError
    {
        DataError::DoesNotExist(DoesNotExistError::Value(t))
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum DoesNotExistError
{
    Tag(Tag),
    Attribute(Tag),
    Condition(Tag),
    Modifier(Tag),
    Equation(Tag),
    Value(Tag),
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct ConflictError
{
    tag: Tag,
    expected: DataType,
    found: DataType,
}

impl ConflictError
{
    pub fn new(tag: Tag, expected: DataType, found: DataType) -> ConflictError
    {
        ConflictError { tag, expected, found }
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum DataType
{
    Tag,
    Attribute,
    Condition,
    Modifier,
    Equation,
}