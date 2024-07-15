use serde::{Deserialize, Serialize};

use crate::model::data_model::{primatives::values::{Value, ValueEffect}, storage::{types::TypeRef, values::ValueRef, view_context::ViewContext, IndexRef, Query, RefTarget, Storable}};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct ModifierType { // What would be ModifierValue? Do we need modifier type? Could just call it modifier?
    pub name: String,
    target_type: TypeRef,
    change: ValueEffect,
}

impl ModifierType {
    pub fn apply(&self, v: Value, context: &ViewContext) -> Value {
        self.change.apply(v, context)
    }
}

impl Storable for ModifierType {
    fn get_container(&self) -> &crate::model::data_model::storage::ContainerKind {
        todo!()
    }
}