use sled::{Db, Tree};

use crate::model::{core::Error, database::{entity::{Entity, EntityID}, Database, DatabaseError}};

/// The handle to the Sled Database
/// What the server has control over
pub struct SledDB
{
    db: Db,
    entities: Tree,
}

impl From<sled::Error> for DatabaseError
{
    fn from(value: sled::Error) -> Self {
        match value
        {
            sled::Error::CollectionNotFound(ivec) => DatabaseError::DatabaseNotFound(format!("{:?}", ivec)),
            sled::Error::Unsupported(s) => DatabaseError::UnsupportedOperation(s),
            sled::Error::ReportableBug(s) => DatabaseError::UnexpectedBehavior(s),
            sled::Error::Io(error) => DatabaseError::FileIO(error),
            sled::Error::Corruption { at, bt } => DatabaseError::Corruption(format!("[Sled] Found corruption at disk partition: {:?}", at)),
        }
    }
}

impl From<sled::Error> for Error
{
    fn from(value: sled::Error) -> Self {
        Error::Database(value.into())
    }
}

impl SledDB
{
    pub fn open<P>(database_path: P) -> Result<SledDB, Error>
    where 
        P: AsRef<std::path::Path>
    {
        let db = sled::open(database_path)?;
        let entities = db.open_tree(b"entities")?;
        Ok(SledDB { db, entities })
    }
}

impl Database for SledDB
{
    fn insert_entity(&self, e: Entity) -> Result<(), DatabaseError>
    {
        todo!()
    }

    fn get_entity(&self, id: &EntityID) -> Result<Entity, DatabaseError>
    {
        todo!()
    }

    fn update_entity(&self, id: &EntityID, n: Entity) -> Result<Entity, DatabaseError>
    {
        todo!()
    }

    fn remove_entity(&self, id: &EntityID) -> Result<Entity, DatabaseError>
    {
        todo!()
    }

    fn generate_id(&self) -> EntityID
    {
        todo!()
    }
}