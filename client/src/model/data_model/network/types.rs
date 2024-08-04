use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::model::data_model::{primatives::types::Type, storage::types::TypeIndex};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypeIndexDataRaw {
    types: HashMap<String, Type>,
}

impl Into<TypeIndex> for TypeIndexDataRaw {
    fn into(self) -> TypeIndex {
        TypeIndex::new(self.types, None)
    }
}