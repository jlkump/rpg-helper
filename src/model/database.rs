use entity::{Entity, EntityID};

use super::core::Error;

pub mod entity;

pub trait Database
{
    fn insert_entity(&mut self, e: Entity) -> Result<(), DatabaseError>;
    fn get_entity(&self, id: &EntityID) -> Result<Entity, DatabaseError>;
    fn modify_entity(&mut self, id: &EntityID, n: Entity) -> Result<Entity, DatabaseError>;
    fn remove_entity(&mut self, id: &EntityID) -> Result<Entity, DatabaseError>;
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