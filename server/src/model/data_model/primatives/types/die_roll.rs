use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::modifier::ModifierType;

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize, Clone)]
pub struct DieRollType { // Defined by 1d4, 4d6, etc. format
    num_dice: u8,
    num_sides: u16,
    special_sides: HashMap<u8, ModifierType>,
}