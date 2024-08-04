use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::model::{data_model::{primatives::wiki::WikiData, storage::wiki::{WikiIndex, WikiPageRef}}, types::WikiPageId};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct WikiIndexDataRaw {
    pages: HashMap<String, WikiData>,
    display_data_paths: HashMap<WikiPageRef, WikiPageId>, // display data is stored on-disk in a ruleset / setting specific folder, where WikiPage data is stored in a file by WikiPageId.

}

impl Into<WikiIndex> for WikiIndexDataRaw {
    fn into(self) -> WikiIndex {
        WikiIndex::new(self.pages, None, self.display_data_paths)
    }
}