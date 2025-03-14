use core::{Component, EntityID, Query};
use std::collections::HashMap;

use crate::database::Database;

pub mod core;
pub mod store;
pub mod storable;

/// The global registry which holds all the components
/// in the ECS system. This is purely a server-side
/// data structure.
pub struct Registry
{
    // The database could be fairly large, thus we can not hold all
    // data for all type, value, wiki, etc. stores in memory.
    // Instead, we hold a HashMap of the recently used Entities and 
    // the data of the retrieved components.
    component_cache: HashMap<EntityID, Component>,
    database: Database,
}

impl Registry
{
    pub fn update_component(&self, id: &EntityID, c: Component)
    {
        todo!()
    }

    pub fn get_component(&self, id: &EntityID) -> Query<Component>
    {
        todo!()
    }
}