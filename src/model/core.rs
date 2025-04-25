use std::future::Future;

use serde::{Deserialize, Serialize};

use super::{database::{entity::{Entity, EntityID}, DatabaseError}, storable::Storable, store::StoreError};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum Error
{
    Database(DatabaseError),
    Store(StoreError)
}

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
/// For example, given a Value Reference, we first look at the ContainerID.
/// The ContainerID querry returns a Ruleset. We then use the returned
/// Ruleset to resolve the path. The path is different for every Store,
/// but is parsed as a string.
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

    /// While users of this library should never need to access the EntityIDs
    /// for references, it still might be useful for debugging. As such,
    /// this will not be pub(crate) quite yet.
    pub fn get_container_id(&self) -> &EntityID
    {
        &self.container_id
    }

    pub fn get_path(&self) -> &str
    {
        &self.path
    }
}

/// The DataHandle interface is used by all users of this library
/// to interact with the data model defined in Stores and Containers.
pub trait DataHandle
{
    // TODO:
    // - [ ] Define the actual interface we want here. In particular, 
    //       get rid of references to the Entity type and use more direct references to Rulesets, Settings, Games, and Characters
    //       as well as all the Store types.
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

/// A simple util interface implemented on all Databases, Stores, and Containers to make queries on their data easier.
/// 
/// This is used to propigate the functionality of filter and filter_map to the `DataHandle` trait.
/// 
/// TODO: Consider, would it be better to require an iterable implementation on all Databases, Stores, and Containers?
pub(crate) trait Filterable<I>
{
    fn filter<F: Fn(&I) -> bool>(&self, f: F) -> Result<Vec<I>, Error>;
    fn filter_map<T, F: Fn(I) -> Option<T>>(&self, f: F) -> Result<Vec<T>, Error>;
}