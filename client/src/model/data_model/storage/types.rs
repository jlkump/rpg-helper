use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::model::data_model::primatives::{input::InputRequest, types::{boolean::BooleanType, die_roll::DieRollType, enumeration::EnumerationType, equation::Equation, meta::MetaType, modifier::ModifierType, number::NumberType, Type}};

use super::{view_context::ViewContext, ContainerKind, IndexRef, IndexStorage, Query, RefTarget};

#[derive(Debug, PartialEq, Clone)]
pub struct TypeIndex {
    types: HashMap<String, Type>,
    view_context: Option<ViewContext>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct TypeRef {
    target: RefTarget,
    type_kind: TypeRefKind,
}

impl From<NumberTypeRef> for TypeRef {
    fn from(value: NumberTypeRef) -> Self {
        TypeRef { target: value.target.clone(), type_kind: TypeRefKind::Num(value) }
    }
}

impl From<BooleanTypeRef> for TypeRef {
    fn from(value: BooleanTypeRef) -> Self {
        TypeRef { target: value.target.clone(), type_kind: TypeRefKind::Bool(value) }
    }
}

impl From<EnumerationTypeRef> for TypeRef {
    fn from(value: EnumerationTypeRef) -> Self {
        TypeRef { target: value.target.clone(), type_kind: TypeRefKind::Enum(value) }
    }
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
    fn get_container(&self) -> &super::ContainerKind {
        todo!()
    }
    
    fn get_ref_name(&self) -> String {
        todo!()
    }
}

impl IndexStorage<Type, TypeRef> for TypeIndex {
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

impl IndexRef<NumberType> for NumberTypeRef {
    fn get_ref_name(&self) -> String {
        todo!()
    }

    fn get_container(&self) -> &super::ContainerKind {
        todo!()
    }
}


/// ---------------- Boolean Type Reference -----------------------
#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct BooleanTypeRef {
    target: RefTarget,
    name_of_type: String,
}

impl IndexRef<BooleanType> for BooleanTypeRef {
    fn get_ref_name(&self) -> String {
        todo!()
    }

    fn get_container(&self) -> &super::ContainerKind {
        todo!()
    }
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
    fn get_container(&self) -> &super::ContainerKind {
        todo!()
    }
    
    fn get_ref_name(&self) -> String {
        todo!()
    }
}

impl IndexStorage<EnumerationType, EnumerationTypeRef> for TypeIndex {
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
    fn get_container(&self) -> &super::ContainerKind {
        todo!()
    }
    
    fn get_ref_name(&self) -> String {
        todo!()
    }
}

impl IndexStorage<MetaType, MetaTypeRef> for TypeIndex {
    fn get(&self, r: &MetaTypeRef) -> Query<&MetaType> {
        todo!()
    }
}

/// ---------------- Equation Type Reference ---------------------
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Clone)]
pub struct EquationRef {
    container: ContainerKind,
    name: String,
}

impl EquationRef {
    pub fn new(equation_name: &str, container: ContainerKind) -> EquationRef {
        EquationRef { container, name: equation_name.to_string() }
    }
}

impl From<Equation> for EquationRef {
    fn from(value: Equation) -> Self {
        todo!()
    }
}

impl IndexRef<Equation> for EquationRef {
    fn get_container(&self) -> &super::ContainerKind {
        &self.container
    }
    
    fn get_ref_name(&self) -> String {
        self.name.clone()
    }
}

impl IndexStorage<Equation, EquationRef> for TypeIndex {
    fn get<'a>(&'a self, r: &EquationRef) -> Query<&'a Equation> {
        todo!()
    }
}


/// ---------------- Die-Roll Type Reference ---------------------
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Clone)]
pub struct DieRollTypeRef {

}

impl DieRollTypeRef {
    pub fn to_input_request(&self, name: &str) -> InputRequest {
        InputRequest::new(name.to_string(), TypeRef { target: self.get_target(), type_kind: TypeRefKind::DieRoll(self.clone()) }, None)
    }
}

impl IndexRef<DieRollType> for DieRollTypeRef {
    fn get_container(&self) -> &super::ContainerKind {
        todo!()
    }
    
    fn get_ref_name(&self) -> String {
        todo!()
    }
}

impl IndexStorage<DieRollType, DieRollTypeRef> for TypeIndex {
    fn get<'a>(&'a self, r: &DieRollTypeRef) -> Query<&'a DieRollType> {
        todo!()
    }
}

// ----------- Modifier Type Ref -----------------
#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct ModifierTypeRef {
    target: RefTarget,
    name: String,
}

impl IndexRef<ModifierType> for ModifierTypeRef {
    fn get_ref_name(&self) -> String {
        todo!()
    }

    fn get_container(&self) -> &super::ContainerKind {
        todo!()
    }
}

impl IndexStorage<ModifierType, ModifierTypeRef> for TypeIndex {
    fn get<'a>(&'a self, r: &ModifierTypeRef) -> Query<&'a ModifierType> {
        todo!()
    }
}
