use std::{collections::HashMap, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::api::{data::{context::Context, effect::Effect, tag::Tag}, rpg::{character::CharacterModification, timeline::Date}, };

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
/// It holds the date it took place and all the modifications performed.
/// NOTE: If event schemas are changed, the associated Event will NOT be changed.
#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Event
{
    pub schema: Tag,        // Reference to the type that made this event
    pub id: Tag,            // The identifier of this event in particular
    pub date: Date,
    pub effects: Vec<CharacterModification>,
    pub ctx: Context,           // This is the additional ctx which was active during
                                // the creation of this event. It should be fairly small, as it
                                // represents values such as the calculation of event values
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