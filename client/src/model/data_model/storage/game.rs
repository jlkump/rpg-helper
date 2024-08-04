use serde::{Deserialize, Serialize};

use crate::model::{data_model::primatives::{location::Location, permissions::GamePermissions, types::{die_roll::DieRollType, enumeration::EnumerationType, equation::Equation, modifier::ModifierType, Type}, values::{meta::MetaInst, Value}, wiki::WikiPage}, types::{CharacterId, GameId, PlayerId, ServerError}};

use super::{character::Character, location::LocationRef, ruleset::Ruleset, setting::Setting, timeline::{Date, Event, EventRef, Timeline}, types::{DieRollTypeRef, EnumerationTypeRef, EquationRef, ModifierTypeRef, TypeRef}, values::{MetaInstRef, ValueRef}, wiki::WikiPageRef, IndexRef, IndexStorage, Query, RefTarget, Storable};


#[derive(Debug, PartialEq, Clone)]
pub struct Game {
    pub id: GameId,
    ruleset: Ruleset,
    setting: Setting,
    game_data: GameplayData,
    gamemaster_data: Option<GameMasterData>, // The data is available only if the player has GM permissions
    characters: Vec<Character>,
    dead_characters: Vec<CharacterId>, // What to do with dead characters? Store just the ID then fetch from database?
    game_permissions: GamePermissions, // The permissons for all data contained in the game. 
                                       // Can only be edited by the GM
}

/// Gameplay data is data that is created during the game for the players to have access to.
/// It is different than the setting changing, as the gameplay data only affects the current game.
/// Changes here will not affect the setting or ruleset.
#[derive(Debug, PartialEq, Clone)]
pub struct GameplayData {
    timeline: Timeline, // These are events that the GM creates.
    current_date: Date,
}

#[derive(Debug, PartialEq, Clone)]
pub struct GameMasterData {
    // The view of data for the game master to manipulate?
    // The Game already has the data for the GameMaster, but this provides extra info
    // Such as the list of pre-determined events for the timeline or player groups for assigning timeline end events
    // or permissions.
    future_events: Timeline,
    character_groups: Vec<Vec<CharacterId>>,
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

    pub fn get_mut_ruleset(&mut self) -> &mut Ruleset {
        &mut self.ruleset
    }

    pub fn get_mut_setting(&mut self) -> &mut Setting {
        &mut self.setting
    }

    pub fn get_mut_gameplay_data(&mut self) -> &mut GameplayData {
        &mut self.game_data
    }

    pub fn get_mut_gamemaster_data(&mut self) -> Option<&mut GameMasterData> {
        self.gamemaster_data.as_mut()
    }
}

// ---------------- Ref Implementations ---------------------------

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

// ------------- Values ---------------

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

impl IndexStorage<MetaInst, MetaInstRef> for Game {
    fn get<'a>(&'a self, r: &MetaInstRef) -> Query<&'a MetaInst> {
        match r.get_target() {
            RefTarget::Playset => todo!(),
            RefTarget::Character(_) => todo!(),
            RefTarget::GameplayData => todo!(),
            RefTarget::GamemasterData => todo!(),
        }
    }
}
// ------------- Types  ---------------

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

impl IndexStorage<DieRollType, DieRollTypeRef> for Game {
    fn get<'a>(&'a self, r: &DieRollTypeRef) -> Query<&'a DieRollType> {
        todo!()
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

impl IndexStorage<ModifierType, ModifierTypeRef> for Game {
    fn get<'a>(&'a self, r: &ModifierTypeRef) -> Query<&'a ModifierType> {
        todo!()
    }
}