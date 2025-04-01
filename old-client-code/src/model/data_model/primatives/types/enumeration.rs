use serde::{Deserialize, Serialize};

use crate::model::data_model::storage::{ContainerKind, Storable};

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct EnumerationType {
    pub container: ContainerKind,
    pub name: String,
    pub types: Vec<String>,
}

impl Storable for EnumerationType {
    fn get_container(&self) -> &ContainerKind {
        &self.container
    }
}