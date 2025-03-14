use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

use crate::model::data_model::{primatives::values::{modifier::Modifier, number::Number, Value}, storage::{types::ModifierTypeRef, view_context::ViewContext, ContainerKind, IndexRef, Query, Storable}};

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct DieRollType { // Defined by 1d4, 4d6, etc. format
    container: ContainerKind,
    pub num_dice: u8,
    pub num_sides: u16,
    pub special_sides: BTreeMap<u16, ModifierTypeRef>, // Using BTreeMap lets us hash the die roll type
}

impl Storable for DieRollType {
    fn get_container(&self) -> &ContainerKind {
        &self.container
    }
}

impl DieRollType {
    pub fn apply_modifier(&self, side_num: u16, context: &ViewContext) -> Query<f32> {
        match self.special_sides.get(&side_num) {
            Some(m_ref) => {
                let m;
                match m_ref.to_ref(context) {
                    Ok(o) => m = o,
                    Err(e) => return Err(e),
                }
                match m.apply(Value::Num(Number::generic(side_num as f32)), context).as_number(context) {
                    Ok(v) => return Ok(v),
                    Err(e) => return Err(e),
                }
            },
            None => return Ok(side_num as f32),
        }
    }
}