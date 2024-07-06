use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct Number {
    name: String,
    value: i32, // Perhaps store floats by dividing by 100?
}