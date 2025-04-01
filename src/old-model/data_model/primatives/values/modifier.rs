use serde::{Deserialize, Serialize};

use crate::model::data_model::{primatives::types::modifier::ModifierType, storage::{types::ModifierTypeRef, values::ValueRef, view_context::ViewContext, IndexRef, Query, Storable}};

use super::Value;

/// Modifiers are stored in the value index. They don't apply automatically and must be fetched
#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct Modifier {
    modifer_type: ModifierTypeRef,
    target: ValueRef,
    src: String, // TODO: Determine actual source type. Probably a reference of some kind
}

impl Modifier {
    pub fn apply(&self, context: &ViewContext) -> Query<Value> {
        Ok(self.modifer_type.to_ref(context)?.apply(self.target.to_ref(context)?.clone(), context))
    }

    pub fn get_type<'a>(&self, context: &ViewContext<'a>) -> Query<&'a ModifierType> {
        self.modifer_type.to_ref(context)
    }
}

impl Storable for Modifier {
    fn get_container(&self) -> &crate::model::data_model::storage::ContainerKind {
        todo!()
    }
}