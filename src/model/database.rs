use entity::{Entity, EntityID};
use serde::{Deserialize, Serialize};

pub mod entity;
pub mod ruleset;

pub type Query<T> = Result<T, QueryError>;

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum QueryError
{
    // Input(EquationCompute),           // Input is required for Querry to be complete
    ContainerNotFound(EntityID, String),
    StorableNotFound(EntityID, String),
}

pub trait Database
{
    fn create_entity(&mut self, e: Entity) -> Result<(), DatabaseError>;
    fn get_entity(&self, id: &EntityID) -> Result<Entity, DatabaseError>;
    fn modify_entity(&mut self, id: &EntityID, n: Entity) -> Result<Entity, DatabaseError>;
    fn remove_entity(&mut self, id: &EntityID) -> Result<Entity, DatabaseError>;
}

pub enum DatabaseError
{
    DuplicateExistingID(Entity)
}