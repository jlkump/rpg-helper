use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::model::data_model::primatives::{input::InputRequest, types::{boolean::BooleanType, die_roll::DieRollType, enumeration::EnumerationType, equation::Equation, list::ListType, meta::MetaType, modifier::ModifierType, number::NumberType, Type}};

use super::{view_context::ViewContext, ContainerKind, IndexRef, IndexStorage, Query, RefTarget, Storable};

#[derive(Debug, PartialEq, Clone)]
pub struct TypeIndex<'a> {
    types: HashMap<String, Type>,
    view_context: Option<ViewContext<'a>>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct TypeReference<T: Storable> {
    container: ContainerKind,
    name: String,
    phantom: std::marker::PhantomData<T>,
}

impl<T: Storable> TypeReference<T> {
    pub fn new(name: &str, container: ContainerKind) -> Self {
        Self {
            container,
            name: name.to_string(),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<T: Storable> IndexRef<T> for TypeReference<T> {
    fn get_ref_name(&self) -> String {
        self.name.clone()
    }

    fn get_container(&self) -> &ContainerKind {
        &self.container
    }
}

pub type NumberTypeRef = TypeReference<NumberType>;
pub type BooleanTypeRef = TypeReference<BooleanType>;
pub type ListTypeRef = TypeReference<ListType>;
pub type EnumerationTypeRef = TypeReference<EnumerationType>;
pub type MetaTypeRef = TypeReference<MetaType>;
pub type EquationRef = TypeReference<Equation>;
pub type DieRollTypeRef = TypeReference<DieRollType>;