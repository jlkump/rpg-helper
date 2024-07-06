use die_roll::DieRollType;
use enumeration::EnumerationType;
use equation::Equation;
use meta::MetaType;
use serde::{Deserialize, Serialize};

use crate::model::data_model::storage::types::{MetaTypeRef, TypeRef};

pub mod die_roll;
pub mod enumeration;
pub mod equation;
pub mod meta;
pub mod modifier;

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum Type { // Important to note. Changing types in-game will be very difficult. Might be best to restrict it to only changing meta, enums, meta-refs, and die-rolls
    Num,
    List(TypeRef),        // Name of type
    Enum(EnumerationType),
    Meta(MetaType),
    Equation(Equation),
    DieRoll(DieRollType),
    MetaRef(MetaTypeRef), // By Name of meta type. Same as type ref, but assumes a return to a MetaType
}