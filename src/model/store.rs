use serde::{Deserialize, Serialize};

use super::{core::{Error, Reference}, database::entity::EntityID, storable::{Referenceable, StorableBuilder}};

pub mod equation;
pub mod event;
pub mod location;
pub mod map;
pub mod types;
pub mod values;
pub mod wiki;

pub trait Store<T, B>
where
    T: Referenceable,
    B: StorableBuilder<T>
{
    // Every Store is able to be Queried for the storables it contains.
    //
    // A Store is primarily used on the Client-side.
    // A client will have a "StoreContext" which acts as the root store
    // from which querries start, similar to the Registry server-side.
    // fn insert<B>(&self, r: B) -> Result<T, Error> where B: StorableBuilder<T>;
    fn get(&self, r: &Reference) -> Result<Option<&T>, Error>;
    fn set(&mut self, r: B) -> Result<Option<T>, Error>;
    fn remove(&mut self, r: &Reference) -> Result<Option<T>, Error>;
    fn get_all(&self) -> Vec<T>;

    // Useful for searching purposes
    fn filter<F: Fn(&T) -> bool>(&self, f: F) -> Result<Vec<&T>, Error>;
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum StoreError
{
    ContainerIDMismatch(EntityID, EntityID), // ContainerID, ReferenceID
    CircularReference(String)
}