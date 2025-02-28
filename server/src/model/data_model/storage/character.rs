use std::collections::HashMap;

use crate::model::data_model::primatives::{permissions::{CharacterId, PlayerId}, types::equation::Equation};

use super::{location::LocationIndex, ruleset::RulesetId, timeline::{Date, Timeline}, values::ValueIndex, wiki::WikiIndex};

#[derive(Debug, PartialEq, Clone)]
pub struct Character<'a> {
    pub creator: PlayerId,
    pub name: String,                          // Name of the character
    pub id: CharacterId,                       // ID for database storage
    wiki_pages: WikiIndex<'a>,                 // Wiki pages the character has made, typically concerning the character
    equations: HashMap<String, Equation>, // For character specific equations
    values: ValueIndex<'a>,                   // Values for the character, such as characteristics, abilities, etc.
    // We will probably need to store starting / base values
    // and current values. This way, the events can modify current
    // values without changing the base values. The user will
    // have the ability to change base values, but will
    // be warned that doing so will make the timeline assume
    // the changed value is the starting value for the character.
        // base_values: ValueIndex,
    
    character_events: Timeline<'a>,     // Events that change the character's stats throughout the game
    date_limit: Date,               // The furthest the player can go forward in time.
    locations: LocationIndex<'a>,       // For character specific locations, such as a laboratory or base
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterTemplate<'a> {
    name_of_template: String,
    values: ValueIndex<'a>,             // Default values for the character, such as characteristics, abilities, etc.
    requires_ruleset: RulesetId,
}