use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::model::{data_model::primatives::wiki::{WikiData, WikiPage}, types::WikiPageId};

use super::{view_context::ViewContext, IndexRef, IndexStorage, Query, RefTarget};

#[derive(Debug, PartialEq, Clone)]
pub struct WikiIndex {
    pages: HashMap<String, WikiData>,
    view_context: Option<ViewContext>,
    display_data_paths: HashMap<WikiPageRef, WikiPageId>, // display data is stored on-disk in a ruleset / setting specific folder, where WikiPage data is stored in a file by WikiPageId.
}

impl WikiIndex {
    pub fn new(
        pages: HashMap<String, WikiData>, 
        view_context: Option<ViewContext>, 
        display_data_paths: HashMap<WikiPageRef, WikiPageId>
    ) -> WikiIndex {
        WikiIndex { pages, view_context, display_data_paths }
    }

    pub fn set_view_ctx(&mut self, v_ctx: ViewContext) {
        self.view_context = Some(v_ctx);
    }
}

impl IndexStorage<WikiPage, WikiPageRef> for WikiIndex {
    fn get(&self, r: &WikiPageRef) -> Query<&WikiPage> {
        // TODO Break up path by '/' s to get the tree path
        todo!()
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct WikiPageRef {
    // This path of the wiki page is relative to the "virtual" file structure, not the actual structure on disk.
    path: String,
}

impl IndexRef<WikiPage> for WikiPageRef {
    fn get_container(&self) -> &super::ContainerKind {
        todo!()
    }
    
    fn get_ref_name(&self) -> String {
        todo!()
    }
}