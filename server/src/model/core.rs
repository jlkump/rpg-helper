use std::{collections::HashMap, marker::PhantomData};

use serde::{Deserialize, Serialize};

pub type UserID = uuid::Uuid;

pub type EntityID = uuid::Uuid;

/// Entity can be thought of as synonomous with a Store.
/// It is anything that is addressable by an EntityID
/// to the global registry.
pub trait Entity
{
    fn to_id(&self) -> &EntityID;
}

pub type Query<T> = Result<T, QueryError>;

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum QueryError
{
    // Input(EquationCompute),           // Input is required for Querry to be complete
    ContainerNotFound(EntityID, String),
    StorableNotFound(String, String),
}

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

/// This trait is implemented for any type that can be stored in a Store.
/// Any value stored in a store must be able to know how to reference itself
pub trait Storable where Self: std::marker::Sized
{
    // type Ref: Reference<Self>;

    fn to_ref(&self) -> Reference<Self>; //Self::Ref;
}

/// ===== A heirarchical reference =====
///
/// In our heirarchical ECS system, the root database
/// has a number of containers. A storeable ref can reference
/// any value in the system, such as rulesets, settings,
/// games, characters, types, values, etc.
///
/// A Ref is resolved starting at the root database container.
/// The ContainerID identifies a Ruleset, Setting, Game, or Character.
/// From there, the Container itself resolves the path depending on
/// it's implementation of Store<T> for the given type T.
///
/// For example, given a ValueTypeRef, we first look at the ContainerID.
/// The ContainerID querry returns a Ruleset. We then use the returned
/// Ruleset to resolve the path.
pub struct Reference<T>
    where T: Storable
{
    container_id: EntityID,
    path: String,
    _phantom: PhantomData<T>,
}

impl<T> Reference<T>
    where T: Storable
{
    pub fn new(storable: &T) -> Reference<T>
    {
        storable.to_ref()
    }

    pub fn get_container_id(&self) -> &EntityID
    {
        &self.container_id
    }

    pub fn get_path(&self) -> &str
    {
        &self.path
    }
}

/// This is the client-side store for components. This serves as the entry-point
/// for references. This is used for editing rulesets, settings, games, and characters.
pub struct StoreContext
{
    stores: HashMap<EntityID, Component>
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum Component
{
    Container(ContainerComponent),
    Store(StoreComponent),
}

impl Entity for Component
{
    fn to_id(&self) -> &EntityID
    {
        match &self
        {
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
    TypeStore(),
    ValueStore(),
    WikiStore(),
}

impl Entity for StoreComponent
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

impl Entity for ContainerComponent
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