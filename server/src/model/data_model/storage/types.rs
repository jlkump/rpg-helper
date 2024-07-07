use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::model::data_model::primatives::types::{meta::MetaType, Type};

use super::{IndexRef, IndexStorage};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct TypeIndex {
    // game: Option<&'a Game>, // Reference to the current game. Option since it will be None while game is constructed.
    types: HashMap<String, Type>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct TypeRef {
    container: String, // Container could be a setting or ruleset
}

impl IndexRef<Type> for TypeRef {
    fn get_target(&self) -> super::RefTarget {
        todo!()
    }
}

impl IndexStorage<Type, TypeRef> for TypeIndex {
    fn get(&self, r: TypeRef) -> Option<&Type> {
        todo!()
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct MetaTypeRef {
    name: String,
}

impl IndexRef<MetaType> for MetaTypeRef {
    fn get_target(&self) -> super::RefTarget {
        todo!()
    }
}

impl IndexStorage<MetaType, MetaTypeRef> for TypeIndex {
    fn get(&self, r: MetaTypeRef) -> Option<&MetaType> {
        todo!()
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize, Clone)]
pub struct EnumerationTypeRef {

}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Clone)]
pub struct EquationRef {
    
}