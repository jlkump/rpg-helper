use serde::{Deserialize, Serialize};

use super::Value;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct List { // Name a list? Might be useful. For example: Creo::Exp?
    values: Vec<Value>,
}