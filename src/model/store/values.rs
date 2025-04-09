use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::model::{core::{Error, Reference},database::{entity::{Entity, EntityID, StoreComponent}, Database, DatabaseEntity, DatabaseError, DatabaseMutator},storable::{values::{Value, ValueBuilder}, StorableBuilder}};

use super::{Store, StoreError};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct ValueStore
{
    id: EntityID,
    values: HashMap<String, Value>,
}

impl ValueStore
{
    /// Purely for creating ValueStores to do tests with.
    /// Is not expected to exist within a database on disk, only in memory.
    pub fn debug_new() -> ValueStore
    {
        ValueStore {
            id: uuid::Uuid::new_v4(),
            values: HashMap::new(),
        }
    }
}

impl Store<Value, ValueBuilder> for ValueStore {
    fn get(&self, r: &Reference) -> Result<Option<&Value>, Error> {
        if r.get_container_id() != &self.id {
            // Error, incorrect container
            return Err(Error::Store(StoreError::ContainerIDMismatch(
                self.id.clone(),
                r.get_container_id().clone(),
            )));
        }
        
        let full_path = r.get_path();
        
        // Handle array indexing in the path (e.g., "Experience[1]")
        if let Some(bracket_pos) = full_path.find('[') {
            if full_path.ends_with(']') {
                let base_name = &full_path[0..bracket_pos];
                let idx_str = &full_path[bracket_pos + 1..full_path.len() - 1];
                
                if let Some(base_value) = self.values.get(base_name) {
                    if let Ok(idx) = idx_str.parse::<usize>() {
                        // Access the list directly
                        if let crate::model::storable::values::EValue::List(list) = &base_value.data {
                            if idx < list.len() {
                                return Ok(Some(&list[idx]));
                            }
                        }
                    }
                }
                return Ok(None);
            }
        }
        
        // Handle dot notation (e.g., "Magic Theory.Value")
        if let Some(dot_pos) = full_path.find('.') {
            let base_name = &full_path[0..dot_pos];
            let sub_path = &full_path[dot_pos + 1..];
            
            if let Some(base_value) = self.values.get(base_name) {
                return Ok(base_value.get_at_path(sub_path));
            }
            return Ok(None);
        }
        
        // Simple path lookup (e.g., "Strength")
        Ok(self.values.get(full_path))
    }

    fn set(&mut self, r: ValueBuilder) -> Result<Option<Value>, Error> {
        let path = r.name.clone();
        Ok(self.values.insert(path.clone(), r.build(self.id.clone(), path)))
    }

    fn remove(&mut self, r: &Reference) -> Result<Option<Value>, Error> {
        if r.get_container_id() != &self.id {
            Err(Error::Store(StoreError::ContainerIDMismatch(
                self.id.clone(),
                r.get_container_id().clone(),
            )))
        } else {
            // Only support removing entire values, not nested fields
            Ok(self.values.remove(r.get_path()))
        }
    }

    fn get_all(&self) -> Vec<Value> {
        self.values.clone().into_values().collect()
    }
    
    fn filter<F: Fn(&Value) -> bool>(&self, f: F) -> Result<Vec<&Value>, Error>
    {
        Ok(self.values.values().filter(|v| f(*v)).collect())
    }
}

// We need to update the StoreError enum to include CircularReference
// This would normally be done in store.rs, but for now we'll just note it
// 
// In a real implementation, we'd add:
// #[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
// pub enum StoreError {
//     ContainerIDMismatch(EntityID, EntityID),
//     CircularReference(String),
// }

pub struct ValueStoreBuilder {
    copy_from: Option<EntityID>,
}

impl From<ValueStore> for Entity {
    fn from(value: ValueStore) -> Self {
        Entity::Store(StoreComponent::ValueStore(value))
    }
}

impl DatabaseEntity<ValueStoreBuilder> for ValueStore {
    fn new() -> ValueStoreBuilder {
        ValueStoreBuilder { copy_from: None }
    }

    fn to_id(&self) -> &EntityID {
        &self.id
    }
}

impl<D: Database> DatabaseMutator<D, ValueStoreBuilder> for ValueStore {
    fn database_insert(db: &D, builder: ValueStoreBuilder) -> Result<EntityID, Error> {
        let vs = ValueStore {
            id: db.generate_id(),
            values: HashMap::new(),
        };
        let id = vs.id.clone();
        
        // TODO: copy the data from an existing value store if needed
        db.insert_entity(Entity::Store(StoreComponent::ValueStore(vs)))?;
        Ok(id)
    }

    fn database_get(db: &D, id: EntityID) -> Result<Option<Self>, Error> {
        if let Some(e) = db.get_entity(&id)? {
            if let Entity::Store(s) = e {
                if let StoreComponent::ValueStore(v) = s {
                    return Ok(Some(v));
                }
            }
            Err(Error::Database(DatabaseError::EntityTypeMismatch))
        } else {
            Err(Error::Database(DatabaseError::NonExistantEntity(id.clone())))
        }
    }

    fn database_update(db: &D, entity: &Self) -> Result<Self, Error> {
        // Check that the entity exists in the database and that it matches the same type
        if let Some(old) = Self::database_get(db, entity.id.clone())? {
            db.update_entity(Entity::Store(StoreComponent::ValueStore(entity.clone())))?;
            Ok(old)
        } else {
            Err(Error::Database(DatabaseError::NonExistantEntity(entity.id.clone())))
        }
    }

    fn database_remove(db: &D, id: EntityID) -> Result<Option<Self>, Error> {
        let old = Self::database_get(db, id.clone())?;
        db.remove_entity(&id)?;
        Ok(old)
    }
}