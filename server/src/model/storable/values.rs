use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum Value {
    // Num(Number),
    // Bool(Bool),
    // List(List),
    // Enum(Enumeration),
    // Meta(MetaInst),
    // Equation(EquationRef),
    // DieRoll(DieRoll), 
    // MetaRef(MetaInstRef),
    Num(),
    Bool(),
    List(),
    Enum(),
    Meta(),
    Equation(),
    DieRoll(), 
    MetaRef(),
}