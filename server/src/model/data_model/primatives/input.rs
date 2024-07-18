use serde::{Deserialize, Serialize};

use crate::model::data_model::storage::types::{EquationRef, TypeRef};

use super::values::Value;

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Input {
    name: String,  // This helps us pair InputRequest to Input for the evaluation of the EvalTree
    value: Value,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct InputRequest {
    name: String,
    requested_type: TypeRef,
    restrictions: Vec<EquationRef>,
}