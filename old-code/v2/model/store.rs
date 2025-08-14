use serde::{Deserialize, Serialize};

use super::{core::{Error, Reference}, database::entity::EntityID, storable::{Referenceable, StorableBuilder}};

pub mod equation;
pub mod event;
pub mod location;
pub mod map;
pub mod types;
pub mod values;
pub mod wiki;

/// Stores hold the actual data that a user cares about for the functioning of the program.
/// Whether that be types, values, events, locations, wiki pages, etc.
/// 
/// Stores can only store data that is Referenceable, meaning that the
/// data knows how to reference itself. Likewise, updating the data
/// in a store requires being able to build a StorableBuilder
/// of the data type in the store.
/// 
/// This indirection using the builder pattern allows for the reference
/// data of the storable to be set by the Store as the Store is the
/// one that actually has the data (The data being the Store's own EntityID).
pub trait Store<T, B>
where
    T: Referenceable,
    B: StorableBuilder<T>
{
    // fn insert<B>(&self, r: B) -> Result<T, Error> where B: StorableBuilder<T>;
    fn get(&self, r: &Reference) -> Result<Option<&T>, Error>;
    fn set(&mut self, r: B) -> Result<Option<T>, Error>;
    fn remove(&mut self, r: &Reference) -> Result<Option<T>, Error>;
    fn get_all(&self) -> Vec<&T>;

    // Useful for searching purposes
    fn filter<F: Fn(&T) -> bool>(&self, f: F) -> Result<Vec<&T>, Error>;
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum StoreError
{
    ContainerIDMismatch(EntityID, EntityID), // ContainerID, ReferenceID
    CircularReference(String)
}