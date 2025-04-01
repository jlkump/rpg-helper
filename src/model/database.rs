use std::collections::HashSet;

use entity::{user::UserID, Entity, EntityID};
use serde::{Deserialize, Serialize};

use super::core::Error;

pub mod entity;

pub trait Database
{
    fn insert_entity(&mut self, e: Entity) -> Result<(), DatabaseError>;
    fn get_entity(&self, id: &EntityID) -> Result<Entity, DatabaseError>;
    fn modify_entity(&mut self, id: &EntityID, n: Entity) -> Result<Entity, DatabaseError>;
    fn remove_entity(&mut self, id: &EntityID) -> Result<Entity, DatabaseError>;
    // TODO: 
    //      We want to have a more abstract interface to the Database.
    //      In particular, we want to be able to say "create Ruleset"
    //      or "add 'Type' to 'Ruleset'"
    //      I'm thinking this will be added to the particular Containers or Stores (see ruleset.rs)
}

pub enum DatabaseError
{
    DuplicateExistingID(Entity)
}

impl From<DatabaseError> for Error
{
    fn from(value: DatabaseError) -> Self {
        Error::Database(value)
    }
}

/// This allows us to track information about the database itself.
/// It counts as an entity in the Database, as all things are entities.
/// 
/// In particular, this links entity ids to their type. 
/// So, if we query all rulesets, we need only look here 
/// rather than look through all entities.
#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct DatabaseRecord
{
    // Assorted Database stuff
    id: EntityID,
    users: HashSet<UserID>,
    
    // Containers
    // Containers own stores
    rulesets: HashSet<EntityID>,
    settings: HashSet<EntityID>,
    games: HashSet<EntityID>,
    characters: HashSet<EntityID>,

    // Stores
    // Owned by containers, hold the actual values that are used for the app
    equation_stores: HashSet<EntityID>,
    event_stores: HashSet<EntityID>,
    location_stores: HashSet<EntityID>,
    map_stores: HashSet<EntityID>,
    type_stores: HashSet<EntityID>,
    value_stores: HashSet<EntityID>,
    wiki_stores: HashSet<EntityID>,
}