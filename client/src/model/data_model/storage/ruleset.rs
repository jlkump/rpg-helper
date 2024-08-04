use crate::model::data_model::primatives::{location::Location, types::Type, wiki::WikiPage};

use super::{location::{LocationIndex, LocationRef}, types::{TypeIndex, TypeRef}, values::ValueIndex, wiki::{WikiIndex, WikiPageRef}, IndexStorage, Query};

pub type RulesetId = uuid::Uuid;

#[derive(Debug, PartialEq, Clone)]
pub struct Ruleset {
    id: RulesetId,
    wiki: WikiIndex,
    types: TypeIndex,
    // presets: ValueIndex<'a>,
    locations: LocationIndex,
}

impl IndexStorage<WikiPage, WikiPageRef> for Ruleset {
    fn get<'a>(&'a self, r: &WikiPageRef) -> Query<&'a WikiPage> {
        self.wiki.get(r)
    }
}

impl IndexStorage<Type, TypeRef> for Ruleset {
    fn get<'a>(&'a self, r: &TypeRef) -> Query<&'a Type> {
        self.types.get(r)
    }
}

impl IndexStorage<Location, LocationRef> for Ruleset {
    fn get<'a>(&'a self, r: &LocationRef) -> Query<&'a Location> {
        self.locations.get(r)
    }
}