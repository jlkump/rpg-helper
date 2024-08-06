use std::{cell::RefCell, collections::HashMap, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::model::{data_model::{primatives::wiki::{WikiData, WikiFolder, WikiPage}, storage::wiki::{WikiIndex, WikiPageRef}}, types::WikiPageId};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct WikiIndexDataRaw {
    pages: HashMap<String, WikiDataRaw>,
    display_data_paths: HashMap<WikiPageRef, WikiPageId>, // display data is stored on-disk in a ruleset / setting specific folder, where WikiPage data is stored in a file by WikiPageId.

}

impl Into<WikiIndex> for WikiIndexDataRaw {
    fn into(self) -> WikiIndex {
        WikiIndex::new(
            self.pages.into_iter().map(|(s, d)| (s, Rc::new(RefCell::new(d.into())))).collect(),
            None,
            self.display_data_paths
        )
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum WikiDataRaw {
    Page(WikiPageDataRaw),
    Folder(WikiFolderDataRaw),
}

impl Into<WikiData> for WikiDataRaw {
    fn into(self) -> WikiData {
        todo!()
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct WikiPageDataRaw {
}

impl Into<WikiPage> for WikiPageDataRaw {
    fn into(self) -> WikiPage {
        todo!()
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct WikiFolderDataRaw {
}

impl Into<WikiFolder> for WikiFolderDataRaw {
    fn into(self) -> WikiFolder {
        todo!()
    }
}