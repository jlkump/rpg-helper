use std::future::Future;

use serde::{Deserialize, Serialize};

use super::{database::{entity::{Entity, EntityID}, DatabaseError}, storable::Storable, store::StoreError};

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
#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Clone)]
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

/// A DataHandle is used to resolve a Reference 
/// and perform updates on Entities and Storables
/// 
/// It is essentially an async interface to a Database.
/// Network logic is abstracted away underneath the
/// implemnetation of the DataHandle trait.
/// 
/// The server must handle authentication and verification such
/// that the reference does not try to access unauthorized data.
pub trait DataHandle
{
    fn generate_id(&mut self) -> impl Future<Output = Result<EntityID, Error>> + Send;

    fn insert_entity(&mut self, e: Entity) -> impl Future<Output = Result<(), Error>> + Send;
    fn get_entity(&mut self, id: &EntityID) -> impl Future<Output = Result<Entity, Error>> + Send;
    fn update_entity(&mut self, e: Entity) -> impl Future<Output = Result<Entity, Error>> + Send;
    fn remove_entity(&mut self, id: &EntityID) -> impl Future<Output = Result<Entity, Error>> + Send;

    fn filter<F>(&self, f: F) -> impl Future<Output = Result<Vec<Entity>, Error>>
        where F: Fn(&Entity) -> bool;
    fn filter_map<T, F>(&self, f: F) -> impl Future<Output = Result<Vec<T>, Error>>
        where F: Fn(Entity) -> Option<T>;
    
    fn resolve_reference(&mut self, r: Reference) -> impl Future<Output = Result<Storable, Error>>;
}