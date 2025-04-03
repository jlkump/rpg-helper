use serde::{Deserialize, Serialize};
use user::User;

use crate::model::store::types::TypeStore;

use super::DatabaseRecord;

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
    fn to_id(&self) -> &EntityID
    {
        match &self
        {
            Self::Database(database_record) => todo!(),
            Self::User(user) => todo!(),
            Self::Container(cc) => cc.to_id(),
            Self::Store(sc) => sc.to_id(),
        }
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
        todo!()
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum ContainerComponent
{
    Ruleset {
        uuid: uuid::Uuid,
        name: String,
        type_store: uuid::Uuid,
        value_store: uuid::Uuid,
        wiki_store: uuid::Uuid,
        location_store: uuid::Uuid,
        map_store: uuid::Uuid,
    },
    Setting {
        uuid: uuid::Uuid,
        name: String,
        type_store: uuid::Uuid,
        value_store: uuid::Uuid,
        wiki_store: uuid::Uuid,
        location_store: uuid::Uuid,
        map_store: uuid::Uuid,
    },
    Game {
        uuid: uuid::Uuid,
        name: String,
        ruleset_id: uuid::Uuid,
        setting_id: uuid::Uuid,
        value_store: uuid::Uuid,
        wiki_store: uuid::Uuid,
        location_store: uuid::Uuid,
        map_store: uuid::Uuid,
        timeline_store: uuid::Uuid,
    },
    Character {
        uuid: uuid::Uuid,
        name: String,
        owner_id: uuid::Uuid,
        game_id: uuid::Uuid,
        value_store: uuid::Uuid,
        wiki_store: uuid::Uuid,
        timeline_store: uuid::Uuid,
    },
}

impl ContainerComponent
{
    fn to_id(&self) -> &EntityID
    {
        match &self
        {
            ContainerComponent::Ruleset { uuid, .. } => uuid,
            ContainerComponent::Setting { uuid, ..  } => uuid,
            ContainerComponent::Game { uuid, .. } => uuid,
            ContainerComponent::Character { uuid, .. } => uuid,
        }
    }
}

impl ContainerComponent
{
    fn type_store(&self) -> Option<&EntityID> {
        todo!()
    }

    fn value_store(&self) -> Option<&EntityID> {
        todo!()
    }

    fn wiki_store(&self) -> Option<&EntityID> {
        todo!()
    }

    fn location_store(&self) -> Option<&EntityID> {
        todo!()
    }

    fn map_store(&self) -> Option<&EntityID> {
        todo!()
    }

    fn timeline_store(&self) -> Option<&EntityID> {
        todo!()
    }
}