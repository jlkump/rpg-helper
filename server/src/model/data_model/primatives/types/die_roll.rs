use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

use crate::model::data_model::{primatives::modifier::{Modifier, ModifierType}, storage::ContainerKind};

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct DieRollType { // Defined by 1d4, 4d6, etc. format
    pub container: ContainerKind,
    num_dice: u8,
    num_sides: u16,
    special_sides: BTreeMap<u8, Modifier>, // Lets us hash the die roll type
}