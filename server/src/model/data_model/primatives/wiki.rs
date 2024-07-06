use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub enum WikiData {
    Page(WikiPage),
    Folder(WikiFolder),
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct WikiPage {
    heading: String,            // Wikipages are identified by heading.
    sub_headings: Vec<String>,  // User can make links by heading and subheading for display. Ex: [[heading#subheading]]
    display_data: String,       // Stored as Markdown text
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct WikiFolder {
    name: String,
    children: Vec<WikiData>,
}