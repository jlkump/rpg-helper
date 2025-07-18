use std::collections::HashMap;

use crate::api::{data::tag::Tag, rpg::{character::CharacterModification, creation::CharacterTemplate, dice::{DiceSet, DieRoll}, location::Location, timeline::DateSpec}};

/// Contains:
///     - The templates for character creation
///         A template for a character defines:
///         - Required attributes and default equations & conditionals
///         - Required text data (and optional text data)
///         - Character options
///             - A title
///             - A description
///             - Changes to the character creation context
///             - Changes to the end character
///     - The templates for equations
///     - The date spec
///     - Event schemas
///     - Locations and maps
///     - Pre-existing characters (like NPCs)
pub struct Ruleset
{
    date_spec: DateSpec,
    die_rolls: DiceSet,
    character_modifications: HashMap<Tag, CharacterModification>,
    character_templates: HashMap<Tag, CharacterTemplate>,
}

impl Ruleset
{
    pub fn new() -> RulesetBuilder
    {
        todo!()
    }

    pub fn from_existing(ruleset: &Ruleset) -> RulesetBuilder
    {
        todo!()
    }
}

pub struct RulesetBuilder
{
    date_spec: Option<DateSpec>,
    character_modifications: HashMap<Tag, CharacterModification>,
    character_templates: HashMap<Tag, CharacterTemplate>,
}

impl RulesetBuilder
{
    pub fn build(self) -> Ruleset
    {
        todo!()
    }
}