use game::Game;
use intermediate_view::IntermediateView;
use serde::{Deserialize, Serialize};
use types::{MetaTypeRef, TypeRef};
use view_context::ViewContext;

use crate::model::types::CharacterId;

use super::primatives::types::equation::{EquationCompute, EvalError};

pub mod character;
pub mod game;
pub mod intermediate_view;
pub mod location;
pub mod playset;
pub mod ruleset;
pub mod setting;
pub mod timeline;
pub mod types;
pub mod values;
pub mod view_context;
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
    Ruleset,
    Setting,
    GameplayData,
    GamemasterData,
    Character(CharacterId),
}

impl From<&ContainerKind> for RefTarget {
    fn from(value: &ContainerKind) -> Self {
        match value {
            ContainerKind::Setting | ContainerKind::Ruleset => RefTarget::Playset,
            ContainerKind::GameplayData => RefTarget::GamemasterData,
            ContainerKind::GamemasterData => RefTarget::GamemasterData,
            ContainerKind::Character(id) => RefTarget::Character(id.clone()),
        }
    }
}

pub type Query<T> = Result<T, QueryError>;

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum QueryError {
    Input(EquationCompute),           // Input is required for Querry to be complete
    DoesNotExist(String, RefTarget),  // Name and RefTarget of reference
    TargetDoesNotExist(RefTarget),
    ViewContextDoesNotExist,
    FieldDoesNotExist(MetaTypeRef, String),
    TypeMismatch(TypeRef, TypeRef),
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

/// This trait is implemented for any type that can be stored in a IndexStorage.
/// Any value that can be stored in an index must know what their container index is for references.
pub trait Storable {
    fn get_container(&self) -> &ContainerKind;
}

/// This trait is implemented for any type that is used to reference Storable instances stored in an IndexStorage.
pub trait IndexRef<T> 
where
    T: Storable
{
    /// A trait used for debugging and display to the user.
    /// Each reference should have a name of the instance to which it refers.
    fn get_ref_name(&self) -> String;

    fn get_container(&self) -> &ContainerKind;

    /// For determining which part of a Game's data to address.
    fn get_target(&self) -> RefTarget {
        self.get_container().into()
    }

    /// Converts the IndexReference to the actual value of the Storable.
    fn to_ref<'a>(&self, context: &ViewContext<'a>) -> Query<&'a T>
    where
        Self: Sized,
        Game<'a>: IndexStorage<T, Self>,
        IntermediateView<'a>: IndexStorage<T, Self>,
    {
        match context {
            ViewContext::GameView(g) => g.get(&self),
            ViewContext::IntermediateView(i) => i.get(&self),
        }
        // view.get(&self)
    }

    /// Helper for converting from a Reference to a DNE exist error for display and debugging.
    fn to_dne_error(&self) -> QueryError {
        QueryError::DoesNotExist(self.get_ref_name(), self.get_target().clone())
    }

    fn to_target_dne_error(&self) -> QueryError {
        QueryError::TargetDoesNotExist(self.get_target().clone())
    }
}

pub trait IndexStorage<T, R>
where
    T: Storable,
    R: IndexRef<T>
{
    /// Returns the value to which the reference refers in this IndexStorage
    /// Errors are of type QueryError
    fn get<'a>(&'a self, r: &R) -> Query<&'a T>;
}