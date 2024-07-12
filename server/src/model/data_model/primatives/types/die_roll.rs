use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

use crate::model::data_model::{primatives::modifier::{Modifier, ModifierType}, storage::{ContainerKind, Storable}};

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct DieRollType { // Defined by 1d4, 4d6, etc. format
    container: ContainerKind,
    pub num_dice: u8,
    pub num_sides: u16,
    pub special_sides: BTreeMap<u8, Modifier>, // Lets us hash the die roll type
}

impl Storable for DieRollType {
    fn get_container(&self) -> &ContainerKind {
        &self.container
    }
}