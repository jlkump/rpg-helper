use std::{collections::HashMap, fmt::Display};

use crate::{data::meta_type::MetaTypeInstance, error::InsertionError};

/// A wrapper struct that handles all the management of type instances for the
/// meta type system. This will be used to hold all the data of a character
pub struct ValueIndex<'a> {
    values: HashMap<String, MetaTypeInstance<'a>>,
}

impl<'b> ValueIndex<'b> {
    pub fn new<'a>() -> ValueIndexBuilder<'a> {
        ValueIndexBuilder {
            values: HashMap::new()
        }
    }

    // pub fn get_value(&self, meta_instance_name: &str, field_name: &str) -> Option<&'b Value> {
    //     self.values.get(meta_instance_name)?.get_field_value(field_name)
    // }

    // pub fn get_mut_value(&mut self, meta_instance_name: &str, field_name: &str) -> Option<&mut Value<'b>> {
    //     self.values.get_mut(meta_instance_name)?.get_mut_field_value(field_name)
    // }

    pub fn get_instance(&self, meta_instance_name: &str) -> Option<&MetaTypeInstance<'b>> {
        self.values.get(meta_instance_name)
    }

    pub fn get_mut_instance(&mut self, meta_instance_name: &str) -> Option<&mut MetaTypeInstance<'b>> {
        self.values.get_mut(meta_instance_name)
    }

    pub fn insert(&mut self, name: &str, inst: MetaTypeInstance<'b>) -> Result<(), InsertionError<String>> {
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
    values: HashMap<String, MetaTypeInstance<'a>>,
}

impl<'b> ValueIndexBuilder<'b> {
    pub fn insert(mut self, name: &str, val: MetaTypeInstance<'b>) -> Result<Self, InsertionError<String>> {
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