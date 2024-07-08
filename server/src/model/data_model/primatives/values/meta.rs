use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::model::data_model::storage::types::MetaTypeRef;

use super::Value;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct MetaInst {
    pub name: String,
    pub t: MetaTypeRef,
    pub fields: HashMap<String, Value>, // FieldName to value
}