use std::collections::HashMap;

use crate::model::{core::{DataHandle, Error, Reference}, database::entity::{Entity, EntityID}, storable::Storable};

use super::CachedData;

pub struct ClientDataHandle
{
    // ruleset_cache: HashMap<EntityID, CachedData<EntityID, Ruleset>>,
    // setting_cache: HashMap<EntityID, CachedData<EntityID, Setting>>,
    // character_cache: HashMap<EntityID, CachedData<EntityID, Character>>,
    // game_cache: HashMap<EntityID, >
    api_url: String,
    entity_cache: HashMap<EntityID, CachedData<Entity>>,
    storable_cache: HashMap<Reference, CachedData<Storable>>,
}

impl DataHandle for ClientDataHandle
{
    async fn generate_id(&mut self) -> Result<EntityID, Error>
    {
        todo!()
    }


    async fn insert_entity(&mut self, e: Entity) -> Result<(), Error> 
    {
        todo!()   
    }

    async fn get_entity(&mut self, id: &EntityID) -> Result<Entity, Error>
    {
        todo!()
    }

    async fn update_entity(&mut self, e: Entity) -> Result<Entity, Error>
    {
        todo!()
    }

    async fn remove_entity(&mut self, id: &EntityID) -> Result<Entity, Error> {
        todo!()    
    }

    async fn filter<F>(&self, f: F) -> Result<Vec<Entity>, Error>
        where F: Fn(&Entity) -> bool 
    {
        todo!()
    }

    async fn filter_map<T, F>(&self, f: F) -> Result<Vec<T>, Error>
        where F: Fn(Entity) -> Option<T>
    {
        todo!()    
    }

    async fn resolve_reference(&mut self, r: Reference) -> Result<Storable, Error>
    {
        todo!()   
    }
}