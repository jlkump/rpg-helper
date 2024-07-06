use serde::{Deserialize, Serialize};

use die_roll::DieRoll;
use enumeration::Enumeration;
use list::List;
use meta::MetaInst;
use number::Number;

use crate::model::data_model::storage::{types::EquationRef, values::MetaInstRef};

pub mod die_roll;
pub mod enumeration;
pub mod list;
pub mod meta;
pub mod number;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum Value {
    Num(Number),
    List(List),
    Enum(Enumeration),
    Meta(MetaInst),
    Equation(EquationRef),
    DieRoll(DieRoll), 
    MetaRef(MetaInstRef),
}