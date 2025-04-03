use serde::{Deserialize, Serialize};
use user::User;

use crate::model::store::types::TypeStore;

use super::{DatabaseEntity, DatabaseRecord};

pub mod ruleset;
pub mod user;


pub type EntityID = uuid::Uuid;

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum Entity
{
    Database(DatabaseRecord),
    User(User),
    Container(ContainerComponent),
    Store(StoreComponent),
    // TODO: Do we store Users and Players as entities here?
}

impl Entity
{
    pub fn from_typestore(ts: TypeStore) -> Entity
    {
        Entity::Store(StoreComponent::TypeStore(ts))
    }

    pub fn try_into_typestore(self) -> Result<TypeStore, Entity>
    {
        if let Entity::Store(StoreComponent::TypeStore(t)) = self
        {
            Ok(t)
        }
        else
        {
            Err(self)
        }
    }
}

impl DatabaseEntity<Entity> for Entity
{
    fn to_id(&self) -> &EntityID
    {
        match &self
        {
            Self::Database(database_record) => &database_record.id,
            Self::User(user) => user.to_id(),
            Self::Container(cc) => todo!(),
            Self::Store(sc) => sc.to_id(),
        }
    }
    
    fn new() -> Entity
    {
        todo!()
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum StoreComponent
{
    EquationStore(),
    EventStore(), // The timeline
    LocationStore(),
    MapStore(),
    TypeStore(TypeStore),
    ValueStore(),
    WikiStore(),
}

impl StoreComponent
{
    fn to_id(&self) -> &EntityID
    {
        match self
        {
            StoreComponent::EquationStore() => todo!(),
            StoreComponent::EventStore() => todo!(),
            StoreComponent::LocationStore() => todo!(),
            StoreComponent::MapStore() => todo!(),
            StoreComponent::TypeStore(type_store) => type_store.to_id(),
            StoreComponent::ValueStore() => todo!(),
            StoreComponent::WikiStore() => todo!(),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum ContainerComponent
{
    Ruleset,
    Setting,
    Game,
    Character,
}