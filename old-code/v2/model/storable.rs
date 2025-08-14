use serde::{Deserialize, Serialize};
use types::Type;
use values::Value;

use super::{core::Reference, database::entity::EntityID};

// pub mod equation;
pub mod event;
pub mod location;
pub mod map;
pub mod types;
pub mod values;
pub mod wiki;

/// This trait is implemented for any type that can be stored in a Store.
/// Any value stored in a store must be able to know how to reference itself
pub trait Referenceable where Self: std::marker::Sized
{
    fn to_ref(&self) -> Reference;
}

pub trait StorableBuilder<T> where 
    Self: std::marker::Sized,
    T: Referenceable
{
    fn build(self, container_id: EntityID, path: String) -> T;
}

/// This type mainly exists for DataHandle to return a concrete type.
/// A separate lesser use is to have cache of storables so that references
/// can be resolved more quickly when values remain the same.
#[derive(Debug, Deserialize, PartialEq, PartialOrd, Serialize, Clone)]
pub enum Storable
{
    Type(Type),
    Value(Value),
    WikiPage,
    Map,
    Location,
    Event,
}