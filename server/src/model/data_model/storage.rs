use game::Game;
use serde::{Deserialize, Serialize};

use super::primatives::{permissions::CharacterId, types::equation::{EquationCompute, EvalError}};

pub mod character;
pub mod game;
pub mod location;
pub mod ruleset;
pub mod setting;
pub mod timeline;
pub mod types;
pub mod values;
pub mod wiki;


/// The target of the Ref, relative to the game
#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub enum RefTarget {
    Playset,
    GameplayData,
    GamemasterData,
    Character(CharacterId),
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub enum ContainerKind {
    Setting,
    Ruleset,
    GameplayData,
    GamemasterData,
    Character(CharacterId),
}

impl From<ContainerKind> for RefTarget {
    fn from(value: ContainerKind) -> Self {
        match value {
            ContainerKind::Setting | ContainerKind::Ruleset => RefTarget::Playset,
            ContainerKind::GameplayData => RefTarget::GamemasterData,
            ContainerKind::GamemasterData => RefTarget::GamemasterData,
            ContainerKind::Character(id) => RefTarget::Character(id),
        }
    }
}

pub type Query<T> = Result<T, QueryError>;

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum QueryError {
    Input(EquationCompute),
    DoesNotExist(String, RefTarget),  // Name and RefTarget of reference
    Eval(EvalError),
}

impl QueryError {
    pub fn is_dne_err(&self) -> bool {
        if let QueryError::DoesNotExist(_, _) = self {
            return true;
        }
        false
    }
}

pub trait IndexRef<T> {
    fn get_target(&self) -> RefTarget; // For determining which part of the Game's data to address
    fn to_ref<'a>(&self, game: &'a Game) -> Query<&'a T> 
    where 
        Self: Sized,
        Game: IndexStorage<T, Self>,
    {
        game.get(&self)
    }
}

pub trait IndexStorage<T, R> 
where
    R: IndexRef<T>
{
    fn get<'a>(&'a self, r: &R) -> Query<&'a T>;
}