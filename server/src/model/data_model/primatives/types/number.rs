use serde::{Deserialize, Serialize};

use crate::model::data_model::storage::{ContainerKind, RefTarget, Storable};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct NumberType {
    pub container: ContainerKind,
    pub name: String, // Example: Exp, Points, etc are all number types
}

impl Storable for NumberType {
    fn get_container(&self) -> &ContainerKind {
        &self.container
    }
}