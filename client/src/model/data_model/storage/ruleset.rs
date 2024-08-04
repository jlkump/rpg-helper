use crate::model::{data_model::primatives::{location::Location, types::Type, wiki::WikiPage}, types::RulesetId};

use super::{character::CharacterTemplate, location::{LocationIndex, LocationRef}, types::{TypeIndex, TypeRef}, values::ValueIndex, wiki::{WikiIndex, WikiPageRef}, IndexStorage, Query};

#[derive(Debug, PartialEq, Clone)]
pub struct Ruleset {
    id: RulesetId,
    wiki: WikiIndex,
    types: TypeIndex,
    locations: LocationIndex,
    character_templates: Vec<CharacterTemplate>,
}

impl Ruleset {
    pub fn new(
        id: RulesetId, 
        wiki: WikiIndex, 
        types: TypeIndex, 
        locations: LocationIndex, 
        character_templates: Vec<CharacterTemplate>
    ) -> Ruleset {
        Ruleset { id, wiki, types, locations, character_templates }
    }

    pub fn get_wiki(&self) -> &WikiIndex {
        &self.wiki
    }

    pub fn get_mut_wiki(&mut self) -> &mut WikiIndex {
        &mut self.wiki
    }
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