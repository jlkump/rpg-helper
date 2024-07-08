use serde::{Deserialize, Serialize};

use crate::model::{data_model::primatives::{location::Location, permissions::{GamePermissions, PlayerId}, types::{enumeration::EnumerationType, equation::Equation, Type}, values::Value, wiki::WikiPage}, types::ServerError};

use super::{character::Character, location::LocationRef, ruleset::Ruleset, setting::Setting, timeline::{Date, Event, EventRef, Timeline}, types::{EnumerationTypeRef, EquationRef, TypeRef}, values::ValueRef, wiki::WikiPageRef, IndexRef, IndexStorage, Query, RefTarget};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Game {
    ruleset: Ruleset,
    setting: Setting,
    game_data: GameplayData,
    characters: Vec<Character>,
    game_permissions: GamePermissions, // The permissons for all data contained in the game. 
                                       // Can only be edited by the GM
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
        todo!()
    }
}

impl IndexStorage<WikiPage, WikiPageRef> for Game {
    fn get(&self, r: &WikiPageRef) -> Query<&WikiPage> {
        match r.get_target() {
            RefTarget::Playset => {
                let res = self.setting.get(r);
                if res.as_ref().is_err_and(|e| e.is_dne_err()) {
                    // If the error is a DNE error, then return the ruleset instead
                    return self.ruleset.get(r);
                }
                res
            },
            RefTarget::GameplayData => todo!(),
            RefTarget::GamemasterData => todo!(),
            RefTarget::Character(_) => todo!(),
        }
    }
}

impl IndexStorage<Value, ValueRef> for Game {
    fn get(&self, r: &ValueRef) -> Query<&Value> {
        match r.get_target() {
            RefTarget::Playset => todo!(),
            RefTarget::Character(_) => todo!(),
            RefTarget::GameplayData => todo!(),
            RefTarget::GamemasterData => todo!(),
        }
    }
}

impl IndexStorage<Type, TypeRef> for Game {
    fn get(&self, r: &TypeRef) -> Query<&Type> {
        match r.get_target() {
            RefTarget::Playset => todo!(),
            RefTarget::Character(_) => todo!(),
            RefTarget::GameplayData => todo!(),
            RefTarget::GamemasterData => todo!(),
        }
    }
}

impl IndexStorage<EnumerationType, EnumerationTypeRef> for Game {
    fn get<'a>(&'a self, r: &EnumerationTypeRef) -> Query<&'a EnumerationType> {
        todo!()
    }
}

impl IndexStorage<Location, LocationRef> for Game {
    fn get(&self, r: &LocationRef) -> Query<&Location> {
        match r.get_target() {
            RefTarget::Playset => todo!(),
            RefTarget::Character(_) => todo!(),
            RefTarget::GameplayData => todo!(),
            RefTarget::GamemasterData => todo!(),
        }
    }
}

impl IndexStorage<Equation, EquationRef> for Game {
    fn get<'a>(&'a self, r: &EquationRef) -> Query<&'a Equation> {
        match r.get_target() {
            RefTarget::Playset => todo!(),
            RefTarget::Character(_) => todo!(),
            RefTarget::GameplayData => todo!(),
            RefTarget::GamemasterData => todo!(),
        }
    }
}

// Gameplay data is data that is created during the game for the players to have access to.
// It is different than the setting changing, as the gameplay data only affects the current game.
// Changes here will not affect the setting or ruleset.
#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct GameplayData {
    timeline: Timeline, // These are events that the GM creates.
    current_date: Date,
    gm_view: Option<GameMasterView>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct GameMasterView {
    future_events: Timeline,
    // The view of data for the game master to manipulate?
    // The Game already has the data for the GameMaster, but this provides extra info
    // Such as the list of pre-determined events for the timeline
}