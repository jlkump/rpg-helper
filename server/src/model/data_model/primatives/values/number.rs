use std::ops::Deref;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Number {
    pub name: String,
    pub value: f32,
}

impl Deref for Number {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}