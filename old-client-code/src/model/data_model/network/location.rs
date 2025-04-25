use serde::{Deserialize, Serialize};

use crate::model::data_model::storage::location::LocationIndex;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LocationIndexDataRaw {

}

impl Into<LocationIndex> for LocationIndexDataRaw {
    fn into(self) -> LocationIndex {
        todo!()
    }
}