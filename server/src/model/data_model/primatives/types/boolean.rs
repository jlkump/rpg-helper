use serde::{Deserialize, Serialize};

use crate::model::data_model::storage::{ContainerKind, Storable};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct BooleanType {
    pub name: String,
    container: ContainerKind,
}

impl Storable for BooleanType {
    fn get_container(&self) -> &ContainerKind {
        &self.container
    }
}