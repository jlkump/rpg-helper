use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::model::data_model::{primatives::values::{modifier::Modifier, Value}, storage::values::{ValueIndex, ValueRef}};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ValueIndexDataRaw {
    values: HashMap<String, Value>,
    modifiers: HashMap<ValueRef, Vec<Modifier>>, // Modifiers are not actually stored as a variant of Value
}

impl Into<ValueIndex> for ValueIndexDataRaw {
    fn into(self) -> ValueIndex {
        ValueIndex::new(self.values, self.modifiers, None)
    }
}