use serde::{Deserialize, Serialize};

use crate::model::{data_model::storage::ruleset::Ruleset, types::RulesetId};

use super::{character::CharacterTemplateDataRaw, location::LocationIndexDataRaw, types::TypeIndexDataRaw, wiki::WikiIndexDataRaw};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RulesetDataRaw {
    id: RulesetId,
    wiki: WikiIndexDataRaw,
    types: TypeIndexDataRaw,
    locations: LocationIndexDataRaw,
    character_templates: Vec<CharacterTemplateDataRaw>,
}

impl Into<Ruleset> for RulesetDataRaw {
    fn into(self) -> Ruleset {
        Ruleset::new(self.id, self.wiki.into(), self.types.into(), self.locations.into(), self.character_templates.into_iter().map(|f| f.into()).collect())
    }
}