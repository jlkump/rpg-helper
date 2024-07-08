use serde::{Deserialize, Serialize};

use crate::model::data_model::storage::types::TypeRef;

use super::values::ValueEffect;

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct ModifierType { // What would be ModifierValue? Do we need modifier type? Could just call it modifier?
    name: String,
    target: TypeRef,
    change: ValueEffect,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct Modifier {

}

// This is possibility. Really only change is that it prevents the target from being stored.
// Also stores the source of the modifier.
// pub struct ModifierTypeRef {

// }

// pub struct Modifier {
//     t: ModifierTypeRef,
//     src: SourceRef, // dyn IndexRef instead?
//     change: i32,
// }