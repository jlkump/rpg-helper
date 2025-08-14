use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct Bool {
    value: bool,
}

impl Bool {
    pub fn generic(b: bool) -> Bool {
        Bool {
            value: b
        }
    }

    pub fn set_value(&mut self, b: bool) {
        self.value = b;
    }
}

impl Deref for Bool {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}