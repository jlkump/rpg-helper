use serde::{Deserialize, Serialize};

use crate::model::data_model::storage::types::EnumerationTypeRef;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Enumeration {
    pub t: EnumerationTypeRef, // Enumeration Types are stored in the typeIndex
    pub inst: usize,
}