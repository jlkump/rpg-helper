use serde::{Deserialize, Serialize};

use crate::model::data_model::storage::types::DieRollTypeRef;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct DieRoll {
    t: DieRollTypeRef,
    die_results: Vec<u16>,
}