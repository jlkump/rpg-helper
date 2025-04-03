use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::model::{core::{Error, Reference}, database::{entity::{Entity, EntityID, StoreComponent}, Database, DatabaseEntity, DatabaseEntityBuilder, DatabaseError, DatabaseID}, storable::{types::{Type, TypeBuilder}, StorableBuilder}};

use super::{Store, StoreError};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct TypeStore 
{
    id: EntityID,
    types: HashMap<String, Type>,
}

impl TypeStore
{
    /// Purely for creating TypeStores to do tests with.
    /// Is not expected to exist within a database on disk, only in memory.
    pub fn debug_new() -> TypeStore
    {
        TypeStore { id: uuid::Uuid::new_v4(), types: HashMap::new() }
    }
}

impl Store<Type, TypeBuilder> for TypeStore 
{
    fn get(&self, r: &Reference<Type>) -> Result<Option<&Type>, Error> 
    {
        if r.get_container_id() != &self.id
        {
            // Error, incorrect container
            Err(Error::Store(StoreError::ContainerIDMismatch(self.id.clone(), r.get_container_id().clone())))
        }
        else 
        {
            Ok(self.types.get(r.get_path()))
        }
    }
    
    fn set(&mut self, r: TypeBuilder) -> Result<Option<Type>, Error>
    {
        let path = r.name.clone();
        Ok(self.types.insert(r.name.clone(), r.build(self.id.clone(), path)))
    }

    fn remove(&mut self, r: &Reference<Type>) -> Result<Option<Type>, Error> 
    {
        if r.get_container_id() != &self.id
        {
            Err(Error::Store(StoreError::ContainerIDMismatch(self.id.clone(), r.get_container_id().clone())))
        }
        else 
        {
            Ok(self.types.remove(r.get_path()))
        }
    }
}

pub struct TypeStoreBuilder
{
    copy_from: Option<EntityID>
}

impl<D: Database> DatabaseEntityBuilder<D, TypeStore> for TypeStoreBuilder {}

impl DatabaseID for TypeStore
{
    fn to_id(&self) -> &EntityID {
        &self.id
    }
}

impl<D: Database> DatabaseEntity<D, TypeStoreBuilder> for TypeStore
{
    fn new() -> TypeStoreBuilder {
        TypeStoreBuilder { copy_from: None }
    }

    fn database_insert(db: &D, builder: TypeStoreBuilder) -> Result<EntityID, Error> 
    {
        let ts = TypeStore
        {
            id: db.generate_id(),
            types: HashMap::new(),
        };
        let id = ts.id.clone();
        // TODO: copy the data from an existing type store to this one
        db.insert_entity(Entity::Store(StoreComponent::TypeStore(ts)))?;
        Ok(id)
    }

    fn database_get(db: &D, id: EntityID) -> Result<Self, Error>
    {
        if let Ok(e) = db.get_entity(&id)
        {
            if let Entity::Store(s) = e 
            {
                if let StoreComponent::TypeStore(t) = s
                {
                    return Ok(t);
                }
            }
            Err(Error::Database(DatabaseError::EntityTypeMismatch))
        }
        else
        {
            Err(Error::Database(DatabaseError::EntityNotFound(id)))
        }
    }

    fn database_update(db: &D, entity: &Self) -> Result<Self, Error>
    {
        // Check that the entity exists in the database and that it matches the same type
        // If it does, then replace the value in the database
        let old = Self::database_get(db, entity.id)?; // If types don't match, we will return an error here
        db.update_entity(&entity.id, Entity::Store(StoreComponent::TypeStore(entity.clone())))?;
        Ok(old)
    }

    fn database_remove(db: &D, id: EntityID) -> Result<Self, Error>
    {
        // Check that there exists such an entity to remove
        let old = Self::database_get(db, id)?; // If types don't match, we will return an error here
        db.remove_entity(&id)?;
        Ok(old)
    }
}