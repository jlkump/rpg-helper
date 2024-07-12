use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::model::data_model::primatives::types::{die_roll::DieRollType, enumeration::EnumerationType, equation::Equation, meta::MetaType, Type};

use super::{view_context::ViewContext, IndexRef, IndexStorage, Query, RefTarget};

#[derive(Debug, PartialEq, Clone)]
pub struct TypeIndex<'a> {
    types: HashMap<String, Type>,
    view_context: Option<ViewContext<'a>>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct TypeRef {
    target: RefTarget,
    type_kind: TypeRefKind,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
enum TypeRefKind {
    Num(NumberTypeRef),
    Bool(BooleanTypeRef),
    List(Box<TypeRef>),
    Enum(EnumerationTypeRef),
    Meta(MetaTypeRef),
    Equation(EquationRef),
    DieRoll(DieRollTypeRef),
    MetaRef(MetaTypeRef),
}

impl IndexRef<Type> for TypeRef {
    fn get_target(&self) -> super::RefTarget {
        todo!()
    }
    
    fn get_ref_name(&self) -> String {
        todo!()
    }
}

impl IndexStorage<Type, TypeRef> for TypeIndex<'_> {
    fn get(&self, r: &TypeRef) -> Query<&Type> {
        todo!()
    }
}

///////////////////////////////////////////
//         Sub-type References           //
///////////////////////////////////////////

/// ------------------ Number Type Reference -----------------------
#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct NumberTypeRef {
    target: RefTarget,
    name_of_type: String,
}


/// ---------------- Boolean Type Reference -----------------------
#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct BooleanTypeRef {
    target: RefTarget,
    name_of_type: String,
}

/// ---------------- Enumeration Type Reference ---------------------
#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct EnumerationTypeRef {
    target: RefTarget,
    name_of_type: String,
}

impl From<&EnumerationType> for EnumerationTypeRef {
    fn from(value: &EnumerationType) -> Self {
        todo!()
    }
}

impl IndexRef<EnumerationType> for EnumerationTypeRef {
    fn get_target(&self) -> RefTarget {
        todo!()
    }
    
    fn get_ref_name(&self) -> String {
        todo!()
    }
}

impl IndexStorage<EnumerationType, EnumerationTypeRef> for TypeIndex<'_> {
    fn get<'a>(&'a self, r: &EnumerationTypeRef) -> Query<&'a EnumerationType> {
        todo!()
    }
}

/// ---------------- Meta-Inst Type Reference ---------------------
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
    fn get_target(&self) -> RefTarget {
        todo!()
    }
    
    fn get_ref_name(&self) -> String {
        todo!()
    }
}

impl IndexStorage<MetaType, MetaTypeRef> for TypeIndex<'_> {
    fn get(&self, r: &MetaTypeRef) -> Query<&MetaType> {
        todo!()
    }
}

/// ---------------- Equation Type Reference ---------------------
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Clone)]
pub struct EquationRef {
    target: RefTarget,
    name: String,
}

impl From<Equation> for EquationRef {
    fn from(value: Equation) -> Self {
        todo!()
    }
}

impl IndexRef<Equation> for EquationRef {
    fn get_target(&self) -> RefTarget {
        todo!()
    }
    
    fn get_ref_name(&self) -> String {
        todo!()
    }
}

impl IndexStorage<Equation, EquationRef> for TypeIndex<'_> {
    fn get<'a>(&'a self, r: &EquationRef) -> Query<&'a Equation> {
        todo!()
    }
}


/// ---------------- Die-Roll Type Reference ---------------------
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Clone)]
pub struct DieRollTypeRef {

}

impl IndexRef<DieRollType> for DieRollTypeRef {
    fn get_target(&self) -> RefTarget {
        todo!()
    }
    
    fn get_ref_name(&self) -> String {
        todo!()
    }
}

impl IndexStorage<DieRollType, DieRollTypeRef> for TypeIndex<'_> {
    fn get<'a>(&'a self, r: &DieRollTypeRef) -> Query<&'a DieRollType> {
        todo!()
    }
}