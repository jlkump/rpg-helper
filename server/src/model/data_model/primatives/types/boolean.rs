use serde::{Deserialize, Serialize};

use crate::model::data_model::storage::ContainerKind;

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct BooleanType {
    pub container: ContainerKind,
    pub name: String,
}