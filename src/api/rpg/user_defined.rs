use std::collections::HashMap;

use crate::api::data::tag::Tag;

/// User defined data is meant for when users wish to define specific values for a ruleset,
/// like time, date, equations, condtionals, etc.

// TODO: This user struct is too general, I should define things more explicitly,
// like dates, events, locations, etc.
// Note: WikiNotes come in the next layer of abstraction and are meant to target things
// by tags, so as long as everything can be addressed by a tag, this should work.
pub struct UserDefinedData
{
    name: Tag,
    structure: HashMap<String, UserType>,
}

pub struct UserDefinedDataInst
{
    type_name: Tag,
    values: HashMap<String, UserValue>,
}

pub enum UserType
{
    Number,
    Name,
}

pub enum UserValue
{
    Number(f32),
    Name(String),
}