use crate::model::data_model::primatives::permissions::GamePermissions;

use super::{character::Character, ruleset::Ruleset, setting::Setting, timeline::{Event, Timeline}};

#[derive(Debug)] // TODO: This is the data that gets sent to the client, so make it serializable and deserializable
pub struct Game {
    ruleset: Ruleset,
    setting: Setting,
    characters: Vec<Character>,
    global_timeline: Timeline,         // These are events that the GM creates.
    game_permissions: GamePermissions, // The permissons for all data contained in the game. 
                                       // Can only be edited by the GM
}

impl Game {
    pub fn get_complete_timeline(&self) -> Timeline {
        // Combines all timelines available.
        // The user can filter the timeline using the Timeline's methods
        todo!()
    }
}

pub struct GameMasterView {
    future_events: Vec<Event>,
    // The view of data for the game master to manipulate?
    // The Game already has the data for the GameMaster, but this provides extra info
    // Such as the list of pre-determined events for the timeline
}