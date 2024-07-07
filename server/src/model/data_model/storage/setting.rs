use serde::{Deserialize, Serialize};

use super::{location::LocationIndex, types::TypeIndex, values::ValueIndex, wiki::WikiIndex};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Setting {
    id: uuid::Uuid,
    wiki: WikiIndex,
    types: TypeIndex,
    presets: ValueIndex,
    locations: LocationIndex,
}