use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::model::data_model::primatives::location::{Location, LocationType, Map};

use super::{IndexRef, IndexStorage};


#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct LocationRef {
    t: LocationType,
    name: String,
}

impl IndexRef<Location> for LocationRef {
    fn get_target(&self) -> super::RefTarget {
        todo!()
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct LocationIndex {
    regions: HashMap<String, Location>,
    sectors: HashMap<String, Location>,
    locales: HashMap<String, Location>,
    landmarks: HashMap<String, Location>,
    maps: HashMap<uuid::Uuid, Map>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct MapRef {
    id: uuid::Uuid,
}

impl IndexRef<Map> for MapRef {
    fn get_target(&self) -> super::RefTarget {
        todo!()
    }
}

impl IndexStorage<Map, MapRef> for LocationIndex {
    fn get(&self, r: MapRef) -> Option<&Map> {
        todo!()
    }
}