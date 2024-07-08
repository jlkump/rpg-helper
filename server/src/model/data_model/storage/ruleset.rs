use serde::{Deserialize, Serialize};

use crate::model::data_model::primatives::wiki::WikiPage;

use super::{location::LocationIndex, types::TypeIndex, values::ValueIndex, wiki::{WikiIndex, WikiPageRef}, IndexRef, IndexStorage, Query};

pub type RulesetId = uuid::Uuid;

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Ruleset {
    id: RulesetId,
    wiki: WikiIndex,
    types: TypeIndex,
    presets: ValueIndex,
    locations: LocationIndex,
}

impl IndexStorage<WikiPage, WikiPageRef> for Ruleset {
    fn get(&self, r: &WikiPageRef) -> Query<&WikiPage> {
        self.wiki.get(r)
    }
}