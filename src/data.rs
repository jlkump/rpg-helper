use std::{collections::HashMap, fmt::Display, mem::take};

use crate::error::{InsertionError, TypeRegistationErr};

use self::{equation::Equation, meta_type::{MetaType, MetaTypeInstance, Type, Value}};

pub mod meta_type;
pub mod equation;
pub mod timeline;

pub struct EquationIndex {
    equations: HashMap<String, Equation>,
}

impl EquationIndex {
    pub fn new() -> EquationIndexBuilder {
        EquationIndexBuilder { equations: HashMap::new() }
    }

    pub fn get_equation(&mut self, for_type: &str) -> Option<Equation> {
        self.equations.remove(for_type)
    }
}

impl Display for EquationIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (s, e) in &self.equations {
            write!(f, "{:>10}: {}\n", s, e)?
        }
        Ok(())
    }
}

pub struct EquationIndexBuilder {
    equations: HashMap<String, Equation>,
}

impl EquationIndexBuilder {
    pub fn insert_equation(&mut self, for_type: &str, e: Equation) {
        self.equations.insert(for_type.to_string(), e);
    }

    pub fn build(self) -> EquationIndex {
        EquationIndex { equations: self.equations }
    }
}

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
    pub fn combine(&mut self, rhs: TypeIndex) -> Vec<MetaType> {
        let mut result = vec![];
        for t in rhs.types {
            if self.types.contains(&t) {
                result.push(t);
            } else {
                self.types.push(t);
            }
        }
        result
    }

    // Returns the type with the given type name, if it exists
    pub fn get_type(&self, type_name: &str) -> Option<&MetaType> {
        self.types.iter().find(|mt| {mt.get_name().eq(type_name)})
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
    pub fn register_type(&mut self, new_type: MetaType) -> Result<(), TypeRegistationErr> {
        if self.types.contains(&new_type) {
            Err(TypeRegistationErr) // Already present inside the type index
        } else {
            self.types.push(new_type);
            Ok(())
        }
    }

    pub fn build(self) -> TypeIndex {
        TypeIndex {
            types: self.types
        }
    }
}

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

    pub fn get_value(&self, meta_instance_name: &str, field_name: &str) -> Option<&'b Value> {
        self.values.get(meta_instance_name)?.get_field_value(field_name)
    }

    pub fn get_mut_value(&mut self, meta_instance_name: &str, field_name: &str) -> Option<&mut Value<'b>> {
        self.values.get_mut(meta_instance_name)?.get_mut_field_value(field_name)
    }

    pub fn get_instance(&self, meta_instance_name: &str) -> Option<&MetaTypeInstance<'b>> {
        self.values.get(meta_instance_name)
    }

    pub fn get_mut_instance(&mut self, meta_instance_name: &str) -> Option<&mut MetaTypeInstance<'b>> {
        self.values.get_mut(meta_instance_name)
    }


    // pub fn set_value(&mut self, meta_instance_name: &str, field_name: &str, field_value: Value<'b>) -> Result<Option<Value<'b>>, InsertionError> {
    //     if let Some(m) = self.values.get_mut(meta_instance_name) {
    //         m.set_field_value(field_name, field_value)
    //     } else {
    //         Err(InsertionError) // Value can't be set b/c meta field instance with the given name doesn't exist
    //     }
    // }

    // // This adds a value to the meta type instace's Value field if the value field is a list
    // // returns error on a type mis-match or when a meta instance or field doesn't exist
    // pub fn append_to_list(&'b mut self, meta_instance_name: &str, field_name: &str, field_value: Value<'b>) -> Result<(), InsertionError> {
    //     if let Some(m) = self.values.get_mut(meta_instance_name) {
    //         if let Some(v) = m.get_mut_field_value(field_name) {
    //             let v_type = v.get_type().clone();
    //             if let Some(old) = v.as_mut_list() {
    //                 if &v_type == field_value.get_type() {
    //                     old.push(field_value);
    //                     Ok(())
    //                 } else {
    //                     Err(InsertionError) // Type mis-match between list and inserted value
    //                 }
    //             } else {
    //                 Err(InsertionError) // Expected list value type
    //             }
    //         } else {
    //             Err(InsertionError) // Meta Instance doesn't have the given field
    //         }
    //     } else {
    //         Err(InsertionError) // Non-existant meta instance
    //     }
    // }

    pub fn add_instance(&mut self, name: &str, inst: MetaTypeInstance<'b>) -> Result<(), InsertionError> {
        if self.values.contains_key(name) {
            Err(InsertionError) // Instance with the given name already exists, redefinition err
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
    pub fn register_value(&mut self, name: &str, val: MetaTypeInstance<'b>) {
        self.values.insert(name.to_string(), val);
    }

    pub fn build(self) -> ValueIndex<'b> {
        ValueIndex {
            values: self.values
        }
    }
}


// TODO: Define DisplayIndex, which
// Maps a type to a DisplayData type
// A DisplayData holds what params are displayed
// A DisplayData also lets the user edit the data (if it is editable)
// Each field will have a "Is Mutable" flag

// TODO: Define a StyleIndex, which
// Which has a list of StyleSheets
// StyleSheets determine how a DisplayData is presented in HTML / CSS