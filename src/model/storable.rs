use super::{core::Reference, database::entity::EntityID};

pub mod equation;
pub mod event;
pub mod location;
pub mod map;
pub mod types;
pub mod values;
pub mod wiki;

/// This trait is implemented for any type that can be stored in a Store.
/// Any value stored in a store must be able to know how to reference itself
pub trait Storable where Self: std::marker::Sized
{
    fn to_ref(&self) -> Reference;
}

pub trait StorableBuilder<T> where 
    Self: std::marker::Sized,
    T: Storable
{
    fn build(self, container_id: EntityID, path: String) -> T;
}