use serde::{Deserialize, Serialize};

use crate::model::{data_model::primatives::{location::Location, permissions::{GamePermissions, PlayerId}, types::Type, values::Value, wiki::WikiPage}, types::ServerError};

use super::{character::Character, location::LocationRef, ruleset::Ruleset, setting::Setting, timeline::{Event, EventRef, Timeline}, types::TypeRef, values::ValueRef, wiki::WikiPageRef, IndexRef, IndexStorage, RefTarget};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Game {
    ruleset: Ruleset,
    setting: Setting,
    characters: Vec<Character>,
    global_timeline: Timeline,         // These are events that the GM creates.
    game_permissions: GamePermissions, // The permissons for all data contained in the game. 
                                       // Can only be edited by the GM
    gm_view: Option<GameMasterView>,
}

impl Game {
    pub fn filter_permissions(&self, player_id: PlayerId) -> Self {
        // Gives back a game struct that contains the data that is available to be viewed by the player
        todo!()
    }

    pub fn get_complete_timeline(&self) -> Timeline {
        // Combines all timelines available.
        // The user can filter the timeline using the Timeline's methods
        todo!()
    }

    // Update the game in memory. The Server will handle auto-saves. Players can force saves
    pub fn type_update(&mut self, target: TypeRef, new_value: Type) {
        todo!()
    }

    pub fn value_update(&mut self, target: ValueRef, new_value: Value) {
        todo!()
    }

    pub fn wiki_update(&mut self, target: WikiPageRef, new_value: WikiPage) {
        todo!()
    }

    pub fn event_update(&mut self, target: EventRef, new_value: Event) {

    }
}

impl IndexStorage<WikiPage, WikiPageRef> for Game {
    fn get(&self, r: WikiPageRef) -> Option<&WikiPage> {
        match r.get_target() {
            RefTarget::Ruleset => self.ruleset.get(r),
            RefTarget::Setting => todo!(),
            RefTarget::Character(_) => todo!(),
            RefTarget::GlobalTimeline => todo!(),
            RefTarget::GamemasterData => todo!(),
        }
    }
}

impl IndexStorage<Value, ValueRef> for Game {
    fn get(&self, r: ValueRef) -> Option<&Value> {
        match r.get_target() {
            RefTarget::Ruleset => todo!(),
            RefTarget::Setting => todo!(),
            RefTarget::Character(_) => todo!(),
            RefTarget::GlobalTimeline => todo!(),
            RefTarget::GamemasterData => todo!(),
        }
    }
}

impl IndexStorage<Type, TypeRef> for Game {
    fn get(&self, r: TypeRef) -> Option<&Type> {
        match r.get_target() {
            RefTarget::Ruleset => todo!(),
            RefTarget::Setting => todo!(),
            RefTarget::Character(_) => todo!(),
            RefTarget::GlobalTimeline => todo!(),
            RefTarget::GamemasterData => todo!(),
        }
    }
}

impl IndexStorage<Location, LocationRef> for Game {
    fn get(&self, r: LocationRef) -> Option<&Location> {
        match r.get_target() {
            RefTarget::Ruleset => todo!(),
            RefTarget::Setting => todo!(),
            RefTarget::Character(_) => todo!(),
            RefTarget::GlobalTimeline => todo!(),
            RefTarget::GamemasterData => todo!(),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct GameMasterView {
    future_events: Timeline,
    // The view of data for the game master to manipulate?
    // The Game already has the data for the GameMaster, but this provides extra info
    // Such as the list of pre-determined events for the timeline
}