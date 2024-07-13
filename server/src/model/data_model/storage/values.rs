use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::model::data_model::primatives::values::{meta::MetaInst, Value};

use super::{IndexRef, view_context::ViewContext};

#[derive(Debug, PartialEq, Clone)]
pub struct ValueIndex<'a> {
    values: HashMap<String, Value>,
    view_context: Option<ViewContext<'a>>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct MetaInstRef { // MetaRef could also be MetaInst
    // Hold data on the ruleset / setting it came from?
    pub type_name: String,
    pub ref_name: String,
}

impl IndexRef<MetaInst> for MetaInstRef {
    fn get_ref_name(&self) -> String {
        todo!()
    }

    fn get_target(&self) -> super::RefTarget {
        todo!()
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Clone)]
pub struct ValueRef {
    name: String,
    field: Option<Box<ValueRef>>, // If Field is None, return what the named value is. Otherwise, drill further down
}

impl IndexRef<Value> for ValueRef {
    fn get_target(&self) -> super::RefTarget {
        todo!()
    }
    
    fn get_ref_name(&self) -> String {
        todo!()
    }
}