use serde::{Deserialize, Serialize};

use crate::model::data_model::storage::{ContainerKind, RefTarget};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct NumberType {
    pub container: ContainerKind,
    pub name: String, // Example: Exp, Points, etc are all number types
}