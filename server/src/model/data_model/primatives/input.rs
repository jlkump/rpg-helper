use crate::model::data_model::storage::types::{EquationRef, TypeRef};

use super::values::Value;

#[derive(Debug, Clone)]
pub struct Input {
    name: String,  // This helps us pair InputRequest to Input for the evaluation of the EvalTree
    value: Value,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InputRequest {
    name: String,
    requested_type: TypeRef,
    restrictions: Vec<EquationRef>,
}