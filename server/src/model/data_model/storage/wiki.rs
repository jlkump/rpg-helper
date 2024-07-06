use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::model::data_model::primatives::wiki::WikiData;

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize, Clone)]
pub struct WikiIndex {
    pages: HashMap<String, WikiData>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize, Clone)]
pub struct WikiPageRef {
    path: String,
}

impl std::ops::Add for WikiIndex {
    type Output = WikiIndex;
    
    // rhs takes priority for keeping duplicates?
    fn add(self, rhs: Self) -> Self::Output {
        let mut res = WikiIndex {
            pages: HashMap::new(),
        };
        for it in self.pages.into_iter().chain(rhs.pages.into_iter()) {
            res.pages.insert(it.0, it.1);
        }
        res
    }
}