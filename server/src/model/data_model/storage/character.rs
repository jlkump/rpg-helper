use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::model::data_model::primatives::types::equation::Equation;

use super::{location::LocationIndex, ruleset::RulesetId, timeline::{Date, Timeline}, values::ValueIndex, wiki::WikiIndex};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Character {
    creator: uuid::Uuid,
    name: String,                         // String of the character
    id: uuid::Uuid,                       // ID for database storage
    wiki_pages: WikiIndex,                // Wiki pages the character has made, typically concerning the character
    equations: HashMap<String, Equation>, // For character specific equations
    values: ValueIndex,                   // Values for the character, such as characteristics, abilities, etc.
    // We will probably need to store starting / base values
    // and current values. This way, the events can modify current
    // values without changing the base values. The user will
    // have the ability to change base values, but will
    // be warned that doing so will make the timeline assume
    // the changed value is the starting value for the character.
        // base_values: ValueIndex,
    
    character_events: Timeline,     // Events that change the character's stats throughout the game
    date_limit: Date,               // The furthest the player can go forward in time.
    locations: LocationIndex,       // For character specific locations, such as a laboratory or base
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct CharacterTemplate {
    name_of_template: String,
    values: ValueIndex,             // Default values for the character, such as characteristics, abilities, etc.
    requires_ruleset: RulesetId,
}