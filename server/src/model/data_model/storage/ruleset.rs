use crate::model::data_model::primatives::{location::Location, types::Type, wiki::WikiPage};

use super::{location::{LocationIndex, LocationRef}, types::{TypeIndex, TypeRef}, values::ValueIndex, wiki::{WikiIndex, WikiPageRef}, IndexStorage, Query};

pub type RulesetId = uuid::Uuid;

#[derive(Debug, PartialEq, Clone)]
pub struct Ruleset<'a> {
    id: RulesetId,
    wiki: WikiIndex<'a>,
    types: TypeIndex<'a>,
    // presets: ValueIndex<'a>,
    locations: LocationIndex<'a>,
}

impl IndexStorage<WikiPage, WikiPageRef> for Ruleset<'_> {
    fn get<'a>(&'a self, r: &WikiPageRef) -> Query<&'a WikiPage> {
        self.wiki.get(r)
    }
}

impl IndexStorage<Type, TypeRef> for Ruleset<'_> {
    fn get<'a>(&'a self, r: &TypeRef) -> Query<&'a Type> {
        self.types.get(r)
    }
}

impl IndexStorage<Location, LocationRef> for Ruleset<'_> {
    fn get<'a>(&'a self, r: &LocationRef) -> Query<&'a Location> {
        self.locations.get(r)
    }
}