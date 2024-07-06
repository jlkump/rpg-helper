use game::Game;
use serde::{Deserialize, Serialize};

use super::primatives::permissions::CharacterId;

pub mod game;
pub mod character;
pub mod ruleset;
pub mod setting;
pub mod timeline;
pub mod types;
pub mod values;
pub mod wiki;


/// The target of the Ref, relative to the game
#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub enum RefTarget {
    Ruleset,
    Setting,
    Character(CharacterId),
    GlobalTimeline,
    GamemasterData,
}

pub trait IndexRef<T> {
    fn get_target(&self) -> RefTarget; // For determining which part of the Game's data to address
    fn get_ref(&self, game: &Game) -> Option<&T>;
}

pub trait IndexStorage<T, R> 
where
    R: IndexRef<T>
{
    fn get(&self, r: R) -> Option<&T>;
}