use serde::{Deserialize, Serialize};

use crate::model::{core::Error, database::{Database, DatabaseEntity, DatabaseEntityBuilder, DatabaseID}};

use super::EntityID;

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Ruleset
{
    pub id: EntityID,
    pub name: String,
    pub type_store: EntityID,
    pub value_store: EntityID,
    pub wiki_store: EntityID,
    pub location_store: EntityID,
    pub map_store: EntityID,
}

impl DatabaseID for Ruleset
{
    fn to_id(&self) -> &EntityID 
    {
        &self.id
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct RulesetBuilder
{
    pub name: String,
    pub type_store: Option<EntityID>,       // If a value is given for these fields,
    pub value_store: Option<EntityID>,      // then that means the user wants to create a
    pub wiki_store: Option<EntityID>,       // ruleset that copies from the given entityID
    pub location_store: Option<EntityID>,   
    pub map_store: Option<EntityID>,
}

impl<D: Database> DatabaseEntityBuilder<D, Ruleset> for RulesetBuilder {}

impl<D: Database> DatabaseEntity<D, RulesetBuilder> for Ruleset
{
    fn new() -> RulesetBuilder
    {
        RulesetBuilder 
        { 
            name: "Default Ruleset Name".to_string(), 
            type_store: None,
            value_store: None,
            wiki_store: None,
            location_store: None,
            map_store: None
        }
    }
    
    /// Given a ruleset to build, this function will create a ruleset
    /// record within the database. The handle of that record will be returned 
    /// (The Ruleset object itself) if the database operation succeeds.
    /// Otherwise it will return an appropriate error.
    /// 
    /// This function assumes that the RulesetBuilder information has 
    /// already been validated. I.E. that the EntityIDs point to what 
    /// type is expected.   
    fn database_insert(db: &D, builder: RulesetBuilder) -> Result<EntityID, Error>
    {
        // TODO:
        //      Assuming that the entityIDs given are formatted correctly,
        //      clone the values of those given entityIDs,
        //
        //      Essentially, we need to create a new type_store, value_store,
        //      wiki_store, location_store, and map_store. For some of these new stores,
        //      we will be copying the values over from previous stores.
        todo!()
    }
    
    fn database_get(db: &D, id: EntityID) -> Result<Self, Error> {
        todo!()
    }
    
    fn database_update(db: &D, entity: &Self) -> Result<Self, Error> {
        todo!()
    }
    
    fn database_remove(db: &D, id: EntityID) -> Result<Self, Error> {
        todo!()
    }
}