use super::{types::TypeIndex, values::ValueIndex, wiki::WikiIndex};

#[derive(Debug)]
pub struct Ruleset {
    id: uuid::Uuid,
    wiki: WikiIndex,
    types: TypeIndex,
    presets: ValueIndex,
}