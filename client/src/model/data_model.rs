use std::collections::HashMap;

use storage::game::Game;

use super::types::GameId;

//////////////////////////////////////
//       Data Model for Tool        //
//////////////////////////////////////
pub mod network;
pub mod storage;
pub mod primatives;

// TODO:
// [ ]. Define a get_game_context that will return the Id of the game currently active.
// [ ]. The server needs to be able to have any number of GameContexts open to handle a specific game
pub fn game_id_to_ref<'a>(id: &GameId) -> &'a Game {
    todo!()
}