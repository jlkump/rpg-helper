use serde::{Deserialize, Serialize};

use crate::api::data::{evaltree::EvalError, tag::Tag};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum DataError
{
    DoesNotExist(DoesNotExistError),
    ConflictingExpectedType(ConflictError),
    InvalidState(String),
    Evaluation(EvalError),
    Parsing(ParseError),
    Tokenization(TokenizationError),
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

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct ParseError
{
    pub string: String,
    pub index_of_error: usize,
    pub error_type: ParseErrorType,
}

impl ParseError
{
    pub fn new(string: String, index_of_error: usize, error_type: ParseErrorType) -> ParseError
    {
        ParseError { string, index_of_error, error_type }
    }
}

impl From<ParseError> for DataError
{
    fn from(value: ParseError) -> Self
    {
        DataError::Parsing(value)
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum ParseErrorType
{
    Tag(TagParseError),
    Evaluation(EvalParseError),
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum TagParseError
{
    TagEmpty,
    SubTagEmpty,
    InvalidCharacter,
    FirstTagNumeric,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum EvalParseError
{
    TokenInvalid,
    NumberMultipleDecimals,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum TokenizationError
{
    ParenthesesPassedAsToken,
    MethodDoesNotExist,
}

impl From<TokenizationError> for DataError
{
    fn from(value: TokenizationError) -> Self
    {
        DataError::Tokenization(value)
    }
}