use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::model::{data_model::{primatives::types::equation::Equation, storage::character::CharacterTemplate}, types::RulesetId};

use super::values::ValueIndexDataRaw;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CharacterTemplateDataRaw {
    name_of_template: String,
    values: ValueIndexDataRaw,             // Default values for the character, such as characteristics, abilities, etc.
    equations: HashMap<String, Equation>, // Default starting equations
    requires_ruleset: RulesetId,
}

impl Into<CharacterTemplate> for CharacterTemplateDataRaw {
    fn into(self) -> CharacterTemplate {
        todo!()
    }
}