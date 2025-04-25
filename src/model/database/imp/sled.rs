use sled::{Db, Tree};
use actix_web::web::Buf;

use crate::model::{core::{Error, Filterable}, database::{entity::{Entity, EntityID}, Database, DatabaseEntity, DatabaseError}};
/// This is the implementation of the interface for a Database as
/// outlined in database.rs.
/// 
/// This is a handle to the SledDB and is what the server has control over.
pub struct SledDB
{
    db: Db,
    entities: Tree,
}

impl From<sled::Error> for DatabaseError
{
    fn from(value: sled::Error) -> Self
    {
        match value
        {
            sled::Error::CollectionNotFound(ivec) => DatabaseError::DatabaseNotFound(format!("{:?}", ivec)),
            sled::Error::Unsupported(s) => DatabaseError::UnsupportedOperation(s),
            sled::Error::ReportableBug(s) => DatabaseError::UnexpectedBehavior(s),
            sled::Error::Io(error) => DatabaseError::FileIO(error.to_string()),
            sled::Error::Corruption { at, .. } => DatabaseError::Corruption(format!("[Sled] Found corruption at disk partition: {:?}", at)),
        }
    }
}

impl From<sled::Error> for Error
{
    fn from(value: sled::Error) -> Self
    {
        Error::Database(value.into())
    }
}

impl From<bincode::Error> for DatabaseError
{
    fn from(value: bincode::Error) -> Self 
    {
        match *value
        {
            bincode::ErrorKind::Io(error) => DatabaseError::FileIO(error.to_string()),
            bincode::ErrorKind::InvalidUtf8Encoding(_) | 
                bincode::ErrorKind::InvalidBoolEncoding(_) | 
                bincode::ErrorKind::InvalidCharEncoding | 
                bincode::ErrorKind::InvalidTagEncoding(_) => DatabaseError::InvalidEncoding,
            bincode::ErrorKind::DeserializeAnyNotSupported => DatabaseError::UnsupportedOperation("[Bincode]: DeserializeAnyNotSupported".to_owned()),
            bincode::ErrorKind::SizeLimit => DatabaseError::SizeLimit,
            bincode::ErrorKind::SequenceMustHaveLength => DatabaseError::InvalidEncoding,
            bincode::ErrorKind::Custom(s) => DatabaseError::Serialization(s),
        }
    }
}

impl From<bincode::Error> for Error
{
    fn from(value: bincode::Error) -> Self
    {
        Error::Database(value.into())
    }
}

impl SledDB
{
    // Methods "open" and "flush" would be good canidates to put in the Database generic interface
    pub fn open<P>(database_path: P) -> Result<SledDB, Error>
    where 
        P: AsRef<std::path::Path>
    {
        let db = sled::open(database_path)?;
        let entities = db.open_tree(b"entities")?;
        Ok(SledDB { db, entities })
    }

    pub fn flush(&self) -> Result<(), Error>
    {
        self.db.flush()?;
        Ok(())
    }

    fn get_data(&self, id: &EntityID) -> Result<Option<Entity>, DatabaseError>
    {
        if let Some(data) = self.entities.get(id)? {
            Ok(Some(bincode::deserialize_from(data.reader())?))
        } else {
            Ok(None)
        }
    }
}

impl Database for SledDB
{
    fn insert_entity(&self, e: Entity) -> Result<(), DatabaseError>
    {
        if let Some(_) = self.get_data(e.to_id())?
        {
            Err(DatabaseError::DuplicateExistingID(e))
        }
        else
        {
            self.entities.insert(e.to_id(), bincode::serialize(&e)?)?;
            Ok(())
        }
    }

    fn get_entity(&self, id: &EntityID) -> Result<Option<Entity>, DatabaseError>
    {
        self.get_data(id)
    }

    fn update_entity(&self, e: Entity) -> Result<Entity, DatabaseError>
    {
        if let Some(d) = self.get_data(e.to_id())?
        {
            self.entities.insert(e.to_id(), bincode::serialize(&e)?)?;
            Ok(d)
        }
        else
        {
            Err(DatabaseError::NonExistantEntity(e.to_id().clone()))
        }
    }

    fn remove_entity(&self, id: &EntityID) -> Result<Option<Entity>, DatabaseError>
    {
        let old = self.entities.remove(id)?;
        if let Some(old) = old
        {
            Ok(Some(bincode::deserialize_from(old.reader())?))
        }
        else 
        {
            Ok(None)    
        }
    }

    fn generate_id(&self) -> EntityID
    {
        uuid::Uuid::new_v4()
    }
}

impl Filterable<Entity> for SledDB
{
    fn filter<F: Fn(&Entity) -> bool>(&self, f: F) -> Result<Vec<Entity>, Error>
    {
        let mut res = vec![];
        for i in self.entities.iter()
        {
            let (_, v) = i?;
            let e: Entity = bincode::deserialize_from(v.reader())?;
            if f(&e)
            {
                res.push(e);
            }
        }

        Ok(res)
    }
    
    fn filter_map<T, F: Fn(Entity) -> Option<T>>(&self, f: F) -> Result<Vec<T>, Error>
    {
        let mut res = vec![];
        for i in self.entities.iter()
        {
            let (_, v) = i?;
            let e: Entity = bincode::deserialize_from(v.reader())?;
            if let Some(e) = f(e)
            {
                res.push(e);
            }
        }

        Ok(res)
    }
}


#[cfg(test)]
mod test
{
    use std::sync::Arc;

    use once_cell::sync::Lazy;

    use crate::model::{core::Reference, database::entity::StoreComponent, storable::types::Type, store::{types::TypeStore, Store}};

    use super::*;

    static DATA: Lazy<Arc<SledDB>> = Lazy::new(|| Arc::new(SledDB::open("./unit_test_database").unwrap()));

    #[test]
    fn insert_record()
    {
        DATA.insert_entity(Entity::Store(StoreComponent::TypeStore(TypeStore::debug_new()))).unwrap();
    }

    #[test]
    fn insert_read_record()
    {
        let mut t = TypeStore::debug_new();
        t.set(Type::new("Test")).unwrap();
        let id = t.to_id().clone();
        DATA.insert_entity(Entity::Store(StoreComponent::TypeStore(t))).unwrap();
        let res = DATA.get_entity(&id).unwrap().unwrap();
        if let Entity::Store(StoreComponent::TypeStore(res)) = res
        {
            assert!(res.get(&Reference::new(id, "Test".to_string())).unwrap().is_some())
        }
        else
        {
            panic!("Entity retrieved is not the one stored")
        }
    }

    #[test]
    fn insert_read_remove_record()
    {
        let mut t = TypeStore::debug_new();
        t.set(Type::new("Test")).unwrap();
        let id = t.to_id().clone();
        DATA.insert_entity(Entity::Store(StoreComponent::TypeStore(t))).unwrap();
        let res = DATA.get_entity(&id).unwrap().unwrap();
        if let Entity::Store(StoreComponent::TypeStore(res)) = res
        {
            assert!(res.get(&Reference::new(id, "Test".to_string())).unwrap().is_some())
        }
        else
        {
            panic!("Entity retrieved is not the one stored")
        }
        DATA.remove_entity(&id).unwrap();
        assert!(DATA.get_entity(&id).unwrap().is_none(), "Data still in the database")
    }
}