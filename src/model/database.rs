use std::collections::HashSet;

use entity::{user::UserID, Entity, EntityID};
use serde::{Deserialize, Serialize};

use super::core::Error;

pub mod entity;
pub mod imp; // Short for implementation

/// The abstract interface implementation requirements for a database.
/// 
/// Under our model, a `Database` facilitates CRUD operations on Entities,
/// can generate a new EntityID, and can be queried using filters for the data of entities.
/// 
/// Future requirements can be added here as needed, but this should be all 
/// that is required for the basics.
pub(crate) trait Database
{
    fn generate_id(&self) -> EntityID;

    fn insert_entity(&self, e: Entity) -> Result<(), DatabaseError>;
    fn get_entity(&self, id: &EntityID) -> Result<Option<Entity>, DatabaseError>;
    fn update_entity(&self, n: Entity) -> Result<Entity, DatabaseError>;
    fn remove_entity(&self, id: &EntityID) -> Result<Option<Entity>, DatabaseError>;
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum DatabaseError
{
    DuplicateExistingID(Entity),
    DuplicateInsert(EntityID),
    NonExistantEntity(EntityID),
    EntityTypeMismatch,
    DatabaseNotFound(String),
    UnsupportedOperation(String),
    UnexpectedBehavior(String),
    InvalidEncoding,
    SizeLimit,
    Serialization(String),
    FileIO(String),
    Corruption(String),
}

/// This simply makes it easier to propigate `DatabaseErrors` to the library `Error` type using the `?` operator
impl From<DatabaseError> for Error
{
    fn from(value: DatabaseError) -> Self
    {
        Error::Database(value)
    }
}

/// This trait is required to be implemented on any `Entity` instance type. It essentially just states
/// that a database entity needs to be able to refer to itself by an ID and that it has a `new()` method
/// that returns a builder `B`. There are no restrictions placed on the builder type, only that it exists.
pub trait DatabaseEntity<B>
{
    fn new() -> B;
    fn to_id(&self) -> &EntityID;
}

/// This trait is implemented on containers and stores to mark that they know how
/// to modify their own data given a database reference.
/// 
/// In inserting into the Database, the DatabaseMutator trait states that
/// the builder defined in `DatabaseEntity<B>` is used.
pub(crate) trait DatabaseMutator<D, B> where Self: std::marker::Sized + DatabaseEntity<B>, D: Database
{
    fn database_insert(db: &D, builder: B) -> Result<EntityID, Error>;
    fn database_get(db: &D, id: EntityID) -> Result<Option<Self>, Error>;
    fn database_update(db: &D, entity: &Self) -> Result<Self, Error>;
    fn database_remove(db: &D, id: EntityID) -> Result<Option<Self>, Error>;
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
    id: EntityID,                   // ID of the DatabaseRecord itself
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