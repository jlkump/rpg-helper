use serde::{Deserialize, Serialize};

use crate::model::data_model::primatives::wiki::WikiPage;

use super::{location::LocationIndex, types::TypeIndex, values::ValueIndex, wiki::{WikiIndex, WikiPageRef}, IndexRef, IndexStorage};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Ruleset {
    id: uuid::Uuid,
    wiki: WikiIndex,
    types: TypeIndex,
    presets: ValueIndex,
    locations: LocationIndex,
}

impl IndexStorage<WikiPage, WikiPageRef> for Ruleset {
    fn get(&self, r: WikiPageRef) -> Option<&WikiPage> {
        self.wiki.get(r)
    }
}