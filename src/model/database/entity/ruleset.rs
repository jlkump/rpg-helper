use serde::{Deserialize, Serialize};

use crate::model::{core::Error, database::Database};

use super::EntityID;

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Ruleset
{
    id: EntityID,
    name: String,
    type_store: EntityID,
    value_store: EntityID,
    wiki_store: EntityID,
    location_store: EntityID,
    map_store: EntityID,
}

impl Ruleset
{
    pub fn create_ruleset<T>(d: &mut T, ) -> Result<Ruleset, Error> where T: Database
    {
        todo!()
    }
}