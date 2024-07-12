use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::model::data_model::primatives::wiki::{WikiData, WikiPage};

use super::{IndexRef, IndexStorage, Query, view_context::ViewContext};

#[derive(Debug, PartialEq, Clone)]
pub struct WikiIndex<'g> {
    pages: HashMap<String, WikiData>,
    view_context: Option<ViewContext<'g>>,
}

impl IndexStorage<WikiPage, WikiPageRef> for WikiIndex<'_> {
    fn get(&self, r: &WikiPageRef) -> Query<&WikiPage> {
        // TODO Break up path by '/' s to get the tree path
        todo!()
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct WikiPageRef {
    path: String,
}

impl IndexRef<WikiPage> for WikiPageRef {
    fn get_target(&self) -> super::RefTarget {
        todo!()
    }
    
    fn get_ref_name(&self) -> String {
        todo!()
    }
}

// impl<'a> std::ops::Add for WikiIndex<'a> {
//     type Output = WikiIndex<'a>;
//     /// When adding two WikiIndexes, the values in each are combined.
//     /// The rhs values take priority over lhs value in cases of a conflict.
//     /// The rhs' view context is also the one used
//     fn add(self, rhs: Self) -> Self::Output {
//         let mut res = WikiIndex {
//             pages: HashMap::new(),
//             view_context: rhs.view_context,
//         };
//         for it in self.pages.into_iter().chain(rhs.pages.into_iter()) {
//             res.pages.insert(it.0, it.1); // Rhs values will override lhs values
//         }
//         res
//     }
// }