use std::fmt::Display;

use crate::{data::meta_type::MetaType, error::{DefinitionError, InsertionError}};

pub struct TypeIndex {
    types: Vec<MetaType>
}

impl TypeIndex {
    pub fn new() -> TypeIndexBuilder {
        TypeIndexBuilder {
            types: vec![]
        }
    }

    // Returns the list of conflicting types
    // Any non-conflicting type is added to the type index
    pub fn combine(&mut self, rhs: TypeIndex) -> Result<(), InsertionError<MetaType>> {
        let mut conflicts = vec![];
        for t in rhs.types {
            if self.types.contains(&t) {
                conflicts.push(t);
            } else {
                self.types.push(t);
            }
        }
        if conflicts.len() > 0 {
            Err(InsertionError::Conflicting(conflicts))
        } else {
            Ok(())
        }
    }

    pub fn insert(&mut self, meta_type: MetaType) -> Result<(), InsertionError<MetaType>> {
        if self.types.contains(&meta_type) {
            Err(InsertionError::Conflicting(vec![meta_type]))
        } else {
            Ok(())
        }
    }

    // Returns the type with the given type name, if it exists
    pub fn get_type(&self, type_name: &str) -> Option<&MetaType> {
        self.types.iter().find(|mt| {mt.get_type_name().eq(type_name)})
    }
}

impl Display for TypeIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for t in &self.types {
            write!(f, "{}\n", t)?
        }
        Ok(())
    }
}

pub struct TypeIndexBuilder {
    types: Vec<MetaType>,
}

impl TypeIndexBuilder {
    pub fn define_type(mut self, new_type: MetaType) -> Result<Self, DefinitionError<MetaType>> {
        if self.types.contains(&new_type) {
            Err(DefinitionError::Redef(new_type)) // Already present inside the type index
        } else {
            self.types.push(new_type);
            Ok(self)
        }
    }

    pub fn build(self) -> TypeIndex {
        TypeIndex {
            types: self.types
        }
    }
}