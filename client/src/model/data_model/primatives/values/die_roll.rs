use serde::{Deserialize, Serialize};

use crate::model::data_model::storage::{types::DieRollTypeRef, view_context::ViewContext, ContainerKind, IndexRef, Query, Storable};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct DieRoll {
    container: ContainerKind,
    t: DieRollTypeRef,
    die_results: Vec<u16>,
}

impl Storable for DieRoll {
    fn get_container(&self) -> &ContainerKind {
        &self.container
    }
}

impl DieRoll {
    pub fn as_number(&self, context: &ViewContext) -> Query<f32> {
        match self.t.to_ref(context) {
            Ok(d_type) => {
                let res = self.die_results.iter().fold(0f32, |acc, x| {
                    match d_type.apply_modifier(*x, context) {
                        Ok(v) => acc + v,
                        Err(_) => acc,
                    }
                });
                return Ok(res);
            },
            Err(e) => return Err(e),
        }
    }
}