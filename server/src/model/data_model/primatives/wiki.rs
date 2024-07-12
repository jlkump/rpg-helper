use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::model::data_model::storage::{ContainerKind, Storable};

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub enum WikiData {
    Page(WikiPage),
    Folder(WikiFolder),
}

impl Storable for WikiData {
    fn get_container(&self) -> &ContainerKind {
        match self {
            WikiData::Page(p) => &p.container,
            WikiData::Folder(f) => &f.container,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct WikiPage {
    heading: String,            // Wikipages are identified by heading.
    sub_headings: Vec<String>,  // User can make links by heading and subheading for display. Ex: [[heading#subheading]]
    display_data: String,       // Stored as Markdown text
    container: ContainerKind,
}

impl Storable for WikiPage {
    fn get_container(&self) -> &ContainerKind {
        &self.container
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct WikiFolder {
    name: String,
    container: ContainerKind,
    children: BTreeMap<String, WikiData>,
}