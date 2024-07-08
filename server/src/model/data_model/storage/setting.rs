use serde::{Deserialize, Serialize};

use crate::model::data_model::primatives::wiki::WikiPage;

use super::{location::LocationIndex, types::TypeIndex, values::ValueIndex, wiki::{WikiIndex, WikiPageRef}, IndexStorage, Query};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Setting {
    id: uuid::Uuid,
    wiki: WikiIndex,
    types: TypeIndex,
    presets: ValueIndex,
    locations: LocationIndex,
}

impl IndexStorage<WikiPage, WikiPageRef> for Setting {
    fn get(&self, r: &WikiPageRef) -> Query<&WikiPage> {
        todo!()
    }
}