use serde::{Deserialize, Serialize};

use crate::model::data_model::storage::{types::NumberTypeRef, ContainerKind, RefTarget, Storable};

use once_cell::sync::Lazy;

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct NumberType {
    pub container: ContainerKind,
    pub name: String, // Example: Exp, Points, etc are all number types
}

impl NumberType {
    // Used to construct the generic number type used for every ruleset.
    pub fn generic() -> &'static NumberType {
        static NUMBER: Lazy<NumberType> = Lazy::new(|| NumberType {
            container: ContainerKind::Ruleset(uuid::Uuid::nil()),
            name: String::from("Number"),
        });
        return &NUMBER;
    }
}

impl Storable for NumberType {
    fn get_container(&self) -> &ContainerKind {
        &self.container
    }
}