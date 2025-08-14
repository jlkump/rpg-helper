use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::model::data_model::storage::{types::MetaTypeRef, values::MetaInstRef, Query, QueryError, Storable};

use super::Value;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct MetaInst {
    pub name: String,
    pub t: MetaTypeRef,
    pub fields: HashMap<String, Value>, // FieldName to value
}

impl MetaInst {
    pub fn get_field(&self, field_name: &str) -> Query<&Value> {
        if let Some(v) = self.fields.get(field_name) {
            Ok(v)
        } else {
            Err(QueryError::FieldDoesNotExist(self.t.clone(), field_name.to_string()))
        }
    }

    pub fn get_value(&self) -> Query<&Value> {
        self.get_field("value")
    }
}

impl Storable for MetaInst {
    fn get_container(&self) -> &crate::model::data_model::storage::ContainerKind {
        todo!()
    }
}