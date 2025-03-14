use serde::{Deserialize, Serialize};

use crate::model::data_model::storage::types::{EquationRef, TypeRef};

use super::values::Value;

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Input {
    pub name: String,  // This helps us pair InputRequest to Input for the evaluation of the EvalTree
    pub value: Value,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct InputRequest {
    pub name: String,
    requested_type: TypeRef,
    restrictions: Option<Vec<EquationRef>>,
}

impl InputRequest {
    pub fn new(name: String, requested_type: TypeRef, restrictions: Option<Vec<EquationRef>>) -> InputRequest {
        InputRequest { name, requested_type, restrictions }
    }
}