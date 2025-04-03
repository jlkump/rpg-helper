use serde::{Deserialize, Serialize};

use super::{database::{entity::EntityID, DatabaseError}, store::StoreError};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum Error
{
    Database(DatabaseError),
    Store(StoreError)
}

/// ===== A heirarchical reference =====
///
/// In our heirarchical ECS system, the root database
/// has a number of containers. A storeable ref can reference
/// any value in the system, such as rulesets, settings,
/// games, characters, types, values, etc.
///
/// A Ref is resolved starting at the root database container.
/// The ContainerID identifies a Ruleset, Setting, Game, or Character.
/// From there, the Container itself resolves the path depending on
/// it's implementation of Store<T> for the given type T.
///
/// For example, given a ValueTypeRef, we first look at the ContainerID.
/// The ContainerID querry returns a Ruleset. We then use the returned
/// Ruleset to resolve the path.
pub struct Reference
{
    container_id: EntityID,
    path: String,
}

impl Reference
{
    pub fn new(container_id: EntityID, path: String) -> Reference
    {
        Reference { container_id, path }
    }

    pub fn get_container_id(&self) -> &EntityID
    {
        &self.container_id
    }

    pub fn get_path(&self) -> &str
    {
        &self.path
    }
}