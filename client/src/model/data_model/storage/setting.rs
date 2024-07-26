use serde::{Deserialize, Serialize};

use crate::model::data_model::primatives::{location::Location, types::Type, values::Value, wiki::{WikiData, WikiPage}};

use super::{location::{LocationIndex, LocationRef}, types::{TypeIndex, TypeRef}, values::{ValueIndex, ValueRef}, wiki::{WikiIndex, WikiPageRef}, IndexRef, IndexStorage, Query, Storable};

#[derive(Debug, PartialEq, Clone)]
pub struct Setting<'a> {
    id: uuid::Uuid,
    wiki: WikiIndex<'a>,
    types: TypeIndex<'a>,
    // presets: ValueIndex<'a>,
    locations: LocationIndex<'a>,
}

impl IndexStorage<WikiPage, WikiPageRef> for Setting<'_> {
    fn get<'a>(&'a self, r: &WikiPageRef) -> Query<&'a WikiPage> {
        self.wiki.get(r)
    }
}

impl IndexStorage<Type, TypeRef> for Setting<'_> {
    fn get<'a>(&'a self, r: &TypeRef) -> Query<&'a Type> {
        self.types.get(r)
    }
}

impl IndexStorage<Location, LocationRef> for Setting<'_> {
    fn get<'a>(&'a self, r: &LocationRef) -> Query<&'a Location> {
        self.locations.get(r)
    }
}