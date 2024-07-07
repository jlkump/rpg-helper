use serde::{Deserialize, Serialize};

use crate::model::data_model::storage::{location::{LocationRef, MapRef}, timeline::EventTypeRef, types::EquationRef, wiki::WikiPageRef};

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct Location {
    name: String,
    location_type: LocationType,
    display_img_src: Option<String>,
    map_region: Option<MapRegion>,
    map_pin: Option<MapPin>,
    provided_events: Vec<EventTypeRef>,
    addition_restrictions: Vec<Vec<EquationRef>>, // Index paired with the event types above.
    children: Vec<LocationRef>, // Locations contained in this location
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub enum LocationType {   // Locations:  Region -> Sector -> Locale -> Landmark
    Region,
    Sector(Option<Box<LocationRef>>), // Parent if it exists
    Locale(Option<Box<LocationRef>>),
    Landmark(Option<Box<LocationRef>>),
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct Map {
    id: uuid::Uuid,
    name: String,
    img_src: String,
    dimen: (i32, i32),
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct MapCoords {
    coords: (i32, i32)
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct MapPin {
    map: MapRef,
    location: LocationRef,
    pin_coords: MapCoords,
    edge_color: String,
    fill_color: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct MapRegion {
    map: MapRef,
    polygon: Vec<MapCoords>,  // Use clip-path(polygon) with absolute positioning: top: 0, left: 0 relative to the map.
    edge_color: String,
    fill_color: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct MapIcon {
    map: MapRef,
    name: String,
    description: Option<String>, // In Markdown?
    details: Option<WikiPageRef>,
    icon_location: MapCoords,
    img_src: String,
    border_color: String,
    // TODO: Size: MatchGrid?
}