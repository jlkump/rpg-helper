use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::model::data_model::storage::{types::BooleanTypeRef, ContainerKind, Storable};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct BooleanType {
    pub name: String,
    container: ContainerKind,
}

impl BooleanType {
    pub fn generic() -> &'static BooleanType {
        static NUMBER: Lazy<BooleanType> = Lazy::new(|| BooleanType {
            container: ContainerKind::Ruleset(uuid::Uuid::nil()),
            name: String::from("Bool"),
        });
        return &NUMBER;
    }
}

impl Storable for BooleanType {
    fn get_container(&self) -> &ContainerKind {
        &self.container
    }
}