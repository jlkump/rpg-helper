use serde::{Deserialize, Serialize};

use crate::model::{data_model::primatives::{location::Location, types::Type, values::Value, wiki::{WikiData, WikiPage}}, types::SettingId};

use super::{location::{LocationIndex, LocationRef}, types::{TypeIndex, TypeRef}, values::{ValueIndex, ValueRef}, wiki::{WikiIndex, WikiPageRef}, IndexRef, IndexStorage, Query, Storable};

#[derive(Debug, PartialEq, Clone)]
pub struct Setting {
    id: SettingId,
    wiki: WikiIndex,
    types: TypeIndex,
    // presets: ValueIndex<'a>,
    locations: LocationIndex,
}

impl IndexStorage<WikiPage, WikiPageRef> for Setting {
    fn get<'a>(&'a self, r: &WikiPageRef) -> Query<&'a WikiPage> {
        self.wiki.get(r)
    }
}

impl IndexStorage<Type, TypeRef> for Setting {
    fn get<'a>(&'a self, r: &TypeRef) -> Query<&'a Type> {
        self.types.get(r)
    }
}

impl IndexStorage<Location, LocationRef> for Setting {
    fn get<'a>(&'a self, r: &LocationRef) -> Query<&'a Location> {
        self.locations.get(r)
    }
}