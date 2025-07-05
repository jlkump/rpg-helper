use std::{collections::HashMap, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::api::{data::{effect::Effect, tag::Tag}, rpg::timeline::Date, };

/// This is what is defined in a ruleset. It represents the effects
/// performed on a character's data.
/// 
/// It holds
///     - A list of effects to apply to the character
///     - The conditions required to see the event schema (for it to be available to be applied)
///     - A name for the event
///     - A tag for identification
#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct EventSchema
{
    name: String,
    tag: Tag,
    effects: Vec<Effect>,
    // abilities: Vec<Ability>,
    conditions: Vec<Tag>, // Addressed by name
}

/// This is an instance of an Event using specifications from the EventSchema.
/// It holds the date it took place
#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Event
{
    pub schema_ref: Tag,
    pub date: Date,
    pub effects: Vec<Effect>,
    // abilities: Vec<Ability>,
    pub conditions: Vec<Tag>,
}

impl PartialOrd for Event
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
    {
        self.date.partial_cmp(&other.date)
    }
}

/// This does not hold instances of events, but instead contains all EventSchemas
#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct EventSet
{
    specs: HashMap<Tag, EventSchema>,
}