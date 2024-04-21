use std::{collections::HashMap, fmt::Display};

use crate::{data::meta_type::{MetaTypeInstance, Type, Value}, error::InsertionError};

use super::type_index::TypeIndex;

/// A wrapper struct that handles all the management of type instances for the
/// meta type system. This will be used to hold all the data of a character
pub struct ValueIndex<'a> {
    values: HashMap<String, Value<'a>>
}

impl<'b> ValueIndex<'b> {
    pub fn new<'a>() -> ValueIndexBuilder<'a> {
        ValueIndexBuilder {
            values: HashMap::new()
        }
    }

    pub fn get_value(&self, value_name: &str) -> Option<&Value<'b>> {
        todo!()
    }

    pub fn get_mut_value(&self, value_name: &str) -> Option<&mut Value<'b>> {
        todo!()
    }

    pub fn get_parent_of(&self, val: &Value<'b>) -> Option<&Value<'b>> {
        todo!()
    }

    pub fn get_values_of_type(&self, t: &Type, types: &TypeIndex) -> Vec<&Value<'b>> {
        todo!()
    }

    // pub fn get_instance(&self, meta_instance_name: &str) -> Option<&MetaTypeInstance<'b>> {
    //     self.values.get(meta_instance_name)
    // }

    // pub fn get_mut_instance(&mut self, meta_instance_name: &str) -> Option<&mut MetaTypeInstance<'b>> {
    //     self.values.get_mut(meta_instance_name)
    // }

    pub fn insert(&mut self, name: &str, inst: Value<'b>) -> Result<(), InsertionError<String>> {
        if self.values.contains_key(name) {
            Err(InsertionError::Conflicting(vec![name.to_string()])) // Instance with the given name already exists
        } else {
            self.values.insert(name.to_string(), inst);
            Ok(())
        }
    }
}

impl Display for ValueIndex<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (s, v) in &self.values {
            write!(f, "{:>10}: {}\n", s, v)?
        }
        Ok(())
    }
}

pub struct ValueIndexBuilder<'a> {
    values: HashMap<String, Value<'a>>,
}

impl<'b> ValueIndexBuilder<'b> {
    pub fn insert(mut self, name: &str, val: Value<'b>) -> Result<Self, InsertionError<String>> {
        if self.values.contains_key(name) {
            Err(InsertionError::Conflicting(vec![name.to_string()]))
        } else {
            self.values.insert(name.to_string(), val);
            Ok(self)
        }
    }

    pub fn build(self) -> ValueIndex<'b> {
        ValueIndex {
            values: self.values
        }
    }
}