use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::model::core::Reference;

#[derive(Debug, Deserialize, PartialEq, PartialOrd, Serialize, Clone)]
pub enum Value {
    // Num(Number),
    // Bool(Bool),
    // List(List),
    // Enum(Enumeration),
    // Meta(MetaInst),
    // Equation(EquationRef),
    // DieRoll(DieRoll), 
    // MetaRef(MetaInstRef),
    Num(f32),
    Bool(bool),
    List(Vec<Value>),
    Enum(String),
    Struct(BTreeMap<String, Value>),
    // Equation(), // Equation Compute?
    DieRoll(), 
    Reference(Reference),                     // Points to a value matching the type of reference
}