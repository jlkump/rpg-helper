use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::model::data_model::primatives::location::{Location, LocationType, Map};

use super::{view_context::ViewContext, IndexRef, IndexStorage, Query, RefTarget};

#[derive(Debug, PartialEq, Clone)]
pub struct LocationIndex {
    regions: HashMap<String, Location>,
    sectors: HashMap<String, Location>,
    locales: HashMap<String, Location>,
    landmarks: HashMap<String, Location>,
    maps: HashMap<uuid::Uuid, Map>,
    view_context: Option<ViewContext>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct LocationRef {
    t: LocationType,
    name: String,
    parent: Option<String>,
}

impl LocationRef {
    fn as_key(&self) -> String {
        if let Some(p) = &self.parent {
            format!("{}-{}", p, &self.name)
        } else {
            self.name.clone()
        }
    }
}

impl IndexRef<Location> for LocationRef {
    fn get_container(&self) -> &super::ContainerKind {
        todo!()
    }
    
    fn get_ref_name(&self) -> String {
        todo!()
    }
}

impl IndexStorage<Location, LocationRef> for LocationIndex {
    fn get<'a>(&'a self, r: &LocationRef) -> Query<&'a Location> {
        let l;
        match r.t {
            LocationType::Region => l = self.regions.get(&r.as_key()),
            LocationType::Sector => l = self.sectors.get(&r.as_key()),
            LocationType::Locale => l = self.locales.get(&r.as_key()),
            LocationType::Landmark => l = self.landmarks.get(&r.as_key()),
        }
        if let Some(l) = l {
            Ok(l)
        } else {
            Err(r.to_dne_error())
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct MapRef {
    id: uuid::Uuid,
}

impl IndexRef<Map> for MapRef {
    fn get_container(&self) -> &super::ContainerKind {
        todo!()
    }
    
    fn get_ref_name(&self) -> String {
        todo!()
    }
}

impl IndexStorage<Map, MapRef> for LocationIndex {
    fn get(&self, r: &MapRef) -> Query<&Map> {
        todo!()
    }
}