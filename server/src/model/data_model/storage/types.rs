use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::model::data_model::primatives::types::{enumeration::EnumerationType, equation::Equation, meta::MetaType, Type};

use super::{IndexRef, IndexStorage, Query, RefTarget};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct TypeIndex {
    // game: Option<&'a Game>, // Reference to the current game. Option since it will be None while game is constructed.
    types: HashMap<String, Type>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct TypeRef {
    target: RefTarget,
    type_kind: TypeKind,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
enum TypeKind {
    Num(NumberTypeRef),
    Bool(BooleanTypeRef),
    List(Box<TypeRef>),
    Enum(EnumerationTypeRef),
    Meta(MetaTypeRef),
    Equation(EquationRef),
    DieRoll(DieRollTypeRef),
    MetaRef(MetaTypeRef),
}

impl TypeRef {
    // pub fn new_num() -> TypeRef {

    // }
}

impl IndexRef<Type> for TypeRef {
    fn get_target(&self) -> super::RefTarget {
        todo!()
    }
}

impl IndexStorage<Type, TypeRef> for TypeIndex {
    fn get(&self, r: &TypeRef) -> Query<&Type> {
        todo!()
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct NumberTypeRef {
    name_of_type: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct BooleanTypeRef {
    name_of_type: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct EnumerationTypeRef {

}

impl From<&EnumerationType> for EnumerationTypeRef {
    fn from(value: &EnumerationType) -> Self {
        todo!()
    }
}

impl IndexRef<EnumerationType> for EnumerationTypeRef {
    fn get_target(&self) -> super::RefTarget {
        todo!()
    }
}

impl IndexStorage<EnumerationType, EnumerationTypeRef> for TypeIndex {
    fn get<'a>(&'a self, r: &EnumerationTypeRef) -> Query<&'a EnumerationType> {
        todo!()
    }
}


#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct MetaTypeRef {
    name: String,
}

impl From<&MetaType> for MetaTypeRef {
    fn from(value: &MetaType) -> Self {
        todo!()
    }
}

impl IndexRef<MetaType> for MetaTypeRef {
    fn get_target(&self) -> super::RefTarget {
        todo!()
    }
}

impl IndexStorage<MetaType, MetaTypeRef> for TypeIndex {
    fn get(&self, r: &MetaTypeRef) -> Query<&MetaType> {
        todo!()
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Clone)]
pub struct EquationRef {
    target: super::RefTarget,
    name: String,
}

impl From<Equation> for EquationRef {
    fn from(value: Equation) -> Self {
        todo!()
    }
}

impl IndexRef<Equation> for EquationRef {
    fn get_target(&self) -> super::RefTarget {
        todo!()
    }
}

impl IndexStorage<Equation, EquationRef> for TypeIndex {
    fn get<'a>(&'a self, r: &EquationRef) -> Query<&'a Equation> {
        todo!()
    }
}


#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Clone)]
pub struct DieRollTypeRef {

}