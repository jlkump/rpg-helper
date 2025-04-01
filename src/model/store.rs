use std::collections::HashMap;

use super::{core::Reference, database::{entity::{Entity, EntityID}, Query}, storable::Storable};

pub mod equation;
pub mod event;
pub mod location;
pub mod map;
pub mod types;
pub mod values;
pub mod wiki;

pub trait Store<T>
where
    T: Storable
{
    // Every Store is able to be Queried for the storables it contains.
    //
    // A Store is primarily used on the Client-side.
    // A client will have a "StoreContext" which acts as the root store
    // from which querries start, similar to the Registry server-side.
    fn query(&self, r: &Reference<T>) -> Query<T>;
}

/// This is the client-side store for components. This serves as the entry-point
/// for references. This is used for editing rulesets, settings, games, and characters.
pub struct StoreContext
{
    stores: HashMap<EntityID, Entity>
}