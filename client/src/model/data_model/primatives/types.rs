use boolean::BooleanType;
use die_roll::DieRollType;
use enumeration::EnumerationType;
use equation::Equation;
use list::ListType;
use meta::MetaType;
use modifier::ModifierType;
use number::NumberType;
use serde::{Deserialize, Serialize};

use crate::model::data_model::storage::{types::{DieRollTypeRef, EnumerationTypeRef, EquationRef, MetaTypeRef, TypeRef}, Storable};

pub mod boolean;
pub mod die_roll;
pub mod enumeration;
pub mod equation;
pub mod list;
pub mod meta;
pub mod modifier;
pub mod number;

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum Type { // Important to note. Changing types in-game will be very difficult. Might be best to restrict it to only changing meta, enums, meta-refs, and die-rolls
    Num(NumberType),
    Bool(BooleanType),
    List(ListType),        // Name of type
    Enum(EnumerationType),
    Meta(MetaType),
    Equation(Equation),
    DieRoll(DieRollType),
    MetaRef(MetaTypeRef), // By Name of meta type. Same as type ref, but assumes a return to a MetaType
    Modifier(ModifierType),
}

impl Storable for Type {
    fn get_container(&self) -> &crate::model::data_model::storage::ContainerKind {
        todo!()
    }
}

impl Type {
    pub fn get_ref(&self) -> TypeRef {
        todo!()
    }
}