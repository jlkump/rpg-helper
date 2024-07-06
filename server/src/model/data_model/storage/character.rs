use serde::{Deserialize, Serialize};

use super::{timeline::{Date, Timeline}, values::ValueIndex, wiki::WikiIndex};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Character {
    creator: uuid::Uuid,
    name: String,                   // String of the character
    id: uuid::Uuid,                 // ID for database storage
    wiki_pages: WikiIndex,          // Wiki pages the character has made, typically concerning the character
    values: ValueIndex,             // Values for the character, such as characteristics, abilities, etc.
    character_events: Timeline,
    date_limit: Date,               // The furthest the player can go forward in time.
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct CharacterTemplate {
    name_of_template: String,
    values: ValueIndex,             // Default values for the character, such as characteristics, abilities, etc.
}