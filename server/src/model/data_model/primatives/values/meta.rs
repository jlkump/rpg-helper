use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::Value;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct MetaInst {
    pub name: String,
    pub fields: HashMap<String, Value>, // FieldName to value
}