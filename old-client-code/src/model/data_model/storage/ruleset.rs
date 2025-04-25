use std::rc::Rc;

use crate::model::{data_model::primatives::{location::Location, types::Type, wiki::WikiPage}, types::RulesetId};

use super::{character::CharacterTemplate, location::{LocationIndex, LocationRef}, types::{TypeIndex, TypeRef}, values::ValueIndex, wiki::{WikiIndex, WikiPageRef}, IndexStorage, Query};

#[derive(Debug, PartialEq, Clone)]
pub struct Ruleset {
    pub id: RulesetId,
    pub display_name: String,
    pub display_description: String,
    pub display_img_src: String,
    wiki: Rc<WikiIndex>,
    types: Rc<TypeIndex>,
    locations: Rc<LocationIndex>,
    character_templates: Vec<Rc<CharacterTemplate>>,
}

impl Ruleset {
    pub fn new(
        id: RulesetId,
        display_name: String,
        display_description: String,
        display_img_src: String, 
        wiki: WikiIndex, 
        types: TypeIndex, 
        locations: LocationIndex, 
        character_templates: Vec<CharacterTemplate>
    ) -> Ruleset {
        Ruleset { 
            id, 
            display_name,
            display_description,
            display_img_src,
            wiki: Rc::new(wiki), 
            types: Rc::new(types), 
            locations: Rc::new(locations), 
            character_templates: character_templates.into_iter().map(|f| Rc::new(f)).collect()
        }
    }

    pub fn get_wiki(&self) -> &WikiIndex {
        &self.wiki
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