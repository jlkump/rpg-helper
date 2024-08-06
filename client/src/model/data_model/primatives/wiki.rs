use std::{cell::RefCell, collections::BTreeMap, rc::{Rc, Weak}};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::model::data_model::storage::{wiki::WikiPageRef, ContainerKind, Storable};

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize, Clone)]
pub enum WikiData {
    Page(WikiPage),
    Folder(WikiFolder),
}

impl WikiData {
    pub fn get_name(&self) -> &str {
        match self {
            WikiData::Page(p) => p.get_name(),
            WikiData::Folder(f) => f.get_name(),
        }
    }

    pub fn get_last_edit_time(&self) -> DateTime<Utc> {
        todo!()
    }
}

impl Storable for WikiData {
    fn get_container(&self) -> &ContainerKind {
        match self {
            WikiData::Page(p) => &p.container,
            WikiData::Folder(f) => &f.container,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize, Clone)]
pub struct WikiPage {
    last_edit: DateTime<Utc>,
    heading: String,            // Wikipages are identified by heading.
    sub_headings: Vec<String>,  // User can make links by heading and subheading for display. Ex: [[heading#subheading]]
    self_ref: WikiPageRef,
    container: ContainerKind,
}

impl WikiPage {
    pub fn get_name(&self) -> &str {
        &self.heading
    }

    pub fn get_last_edit_time(&self) -> DateTime<Utc> {
        todo!()
    }
}

impl Storable for WikiPage {
    fn get_container(&self) -> &ContainerKind {
        &self.container
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize, Clone)]
pub struct WikiFolder {
    last_edit: DateTime<Utc>,
    name: String,
    container: ContainerKind,
    children: Vec<Rc<RefCell<WikiData>>>,
}

impl WikiFolder {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_children(&self) -> Vec<Rc<RefCell<WikiData>>> {
        self.children.clone()
    }

    pub fn get_last_edit_time(&self) -> DateTime<Utc> {
        todo!()
    }
}