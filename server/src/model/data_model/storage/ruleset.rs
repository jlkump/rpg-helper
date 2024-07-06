use serde::{Deserialize, Serialize};

use super::{types::TypeIndex, values::ValueIndex, wiki::WikiIndex};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Ruleset {
    id: uuid::Uuid,
    wiki: WikiIndex,
    types: TypeIndex,
    presets: ValueIndex,
}