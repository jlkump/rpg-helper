use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use crate::model::data_model::storage::types::TypeRef;

use super::Value;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct List { // Name a list? Might be useful. For example: Creo::Exp?
    pub t: TypeRef,
    values: Vec<Value>,
}

impl List {
    pub fn new(t: TypeRef, values: Vec<Value>) -> List {
        List {
            t,
            values
        }
    }
}

impl Deref for List {
    type Target = Vec<Value>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl DerefMut for List {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}