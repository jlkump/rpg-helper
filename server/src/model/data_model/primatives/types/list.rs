use serde::{Deserialize, Serialize};

use crate::model::data_model::storage::{types::TypeRef, ContainerKind, RefTarget};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct ListType {
    pub container: ContainerKind,
    pub t: Box<TypeRef>,
}