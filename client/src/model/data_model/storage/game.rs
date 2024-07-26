use serde::{Deserialize, Serialize};

use crate::model::{data_model::primatives::{location::Location, permissions::{CharacterId, GamePermissions, PlayerId}, types::{die_roll::DieRollType, enumeration::EnumerationType, equation::Equation, modifier::ModifierType, Type}, values::{meta::MetaInst, Value}, wiki::WikiPage}, types::ServerError};

use super::{character::Character, location::LocationRef, ruleset::Ruleset, setting::Setting, timeline::{Date, Event, EventRef, Timeline}, types::{DieRollTypeRef, EnumerationTypeRef, EquationRef, ModifierTypeRef, TypeRef}, values::{MetaInstRef, ValueRef}, wiki::WikiPageRef, IndexRef, IndexStorage, Query, RefTarget, Storable};

pub type GameId = uuid::Uuid;

#[derive(Debug, PartialEq, Clone)]
pub struct Game<'a> {
    id: GameId,
    ruleset: Ruleset<'a>,
    setting: Setting<'a>,
    game_data: GameplayData<'a>,
    gamemaster_data: Option<GameMasterData<'a>>, // The data is available only if the player has GM permissions
    characters: Vec<Character<'a>>,
    dead_characters: Vec<CharacterId>, // What to do with dead characters? Store just the ID then fetch from database?
    game_permissions: GamePermissions, // The permissons for all data contained in the game. 
                                       // Can only be edited by the GM
}

/// Gameplay data is data that is created during the game for the players to have access to.
/// It is different than the setting changing, as the gameplay data only affects the current game.
/// Changes here will not affect the setting or ruleset.
#[derive(Debug, PartialEq, Clone)]
pub struct GameplayData<'a> {
    timeline: Timeline<'a>, // These are events that the GM creates.
    current_date: Date,
}

#[derive(Debug, PartialEq, Clone)]
pub struct GameMasterData<'a> {
    // The view of data for the game master to manipulate?
    // The Game already has the data for the GameMaster, but this provides extra info
    // Such as the list of pre-determined events for the timeline or player groups for assigning timeline end events
    // or permissions.
    future_events: Timeline<'a>,
    character_groups: Vec<Vec<CharacterId>>,
}

impl<'g> Game<'g> {
    pub fn filter_permissions(&self, player_id: PlayerId) -> Self {
        // Gives back a game struct that contains the data that is available to be viewed by the player
        todo!()
    }

    pub fn get_complete_timeline(&self) -> Timeline {
        // Combines all timelines available.
        // The user can filter the timeline using the Timeline's methods
        todo!()
    }

    pub fn get_mut_ruleset(&mut self) -> &mut Ruleset<'g> {
        &mut self.ruleset
    }

    pub fn get_mut_setting(&mut self) -> &mut Setting<'g> {
        &mut self.setting
    }

    pub fn get_mut_gameplay_data(&mut self) -> &mut GameplayData<'g> {
        &mut self.game_data
    }

    pub fn get_mut_gamemaster_data(&mut self) -> Option<&mut GameMasterData<'g>> {
        self.gamemaster_data.as_mut()
    }
}

// ---------------- Ref Implementations ---------------------------

impl IndexStorage<WikiPage, WikiPageRef> for Game<'_> {
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

impl IndexStorage<Location, LocationRef> for Game<'_> {
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

impl IndexStorage<Value, ValueRef> for Game<'_> {
    fn get(&self, r: &ValueRef) -> Query<&Value> {
        match r.get_target() {
            RefTarget::Playset => todo!(),
            RefTarget::Character(_) => todo!(),
            RefTarget::GameplayData => todo!(),
            RefTarget::GamemasterData => todo!(),
        }
    }
}

impl IndexStorage<MetaInst, MetaInstRef> for Game<'_> {
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

impl IndexStorage<Type, TypeRef> for Game<'_> {
    fn get(&self, r: &TypeRef) -> Query<&Type> {
        match r.get_target() {
            RefTarget::Playset => todo!(),
            RefTarget::Character(_) => todo!(),
            RefTarget::GameplayData => todo!(),
            RefTarget::GamemasterData => todo!(),
        }
    }
}

impl IndexStorage<EnumerationType, EnumerationTypeRef> for Game<'_> {
    fn get<'a>(&'a self, r: &EnumerationTypeRef) -> Query<&'a EnumerationType> {
        todo!()
    }
}

impl IndexStorage<DieRollType, DieRollTypeRef> for Game<'_> {
    fn get<'a>(&'a self, r: &DieRollTypeRef) -> Query<&'a DieRollType> {
        todo!()
    }
}

impl IndexStorage<Equation, EquationRef> for Game<'_> {
    fn get<'a>(&'a self, r: &EquationRef) -> Query<&'a Equation> {
        match r.get_target() {
            RefTarget::Playset => todo!(),
            RefTarget::Character(_) => todo!(),
            RefTarget::GameplayData => todo!(),
            RefTarget::GamemasterData => todo!(),
        }
    }
}

impl IndexStorage<ModifierType, ModifierTypeRef> for Game<'_> {
    fn get<'a>(&'a self, r: &ModifierTypeRef) -> Query<&'a ModifierType> {
        todo!()
    }
}