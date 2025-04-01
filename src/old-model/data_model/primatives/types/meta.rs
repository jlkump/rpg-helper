use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::model::data_model::storage::{types::TypeRef, ContainerKind, Storable};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct MetaType {
    pub name: String,
    pub container: ContainerKind,
    pub fields: HashMap<String, TypeRef>, // FieldName to type
}

impl Storable for MetaType {
    fn get_container(&self) -> &ContainerKind {
        &self.container
    }
}