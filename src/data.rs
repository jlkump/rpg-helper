use std::{collections::HashMap, fmt::Display};

use self::meta_type::{MetaType, MetaTypeInstance, Value};

pub mod meta_type;
pub mod equation;
pub mod timeline;

#[derive(Debug)]
pub struct TypeRegistationErr;

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
    pub fn get_type(&self, type_name: &String) -> Option<&MetaType> {
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

/// A simple struct to encapsulate querying values and updating values
/// of meta type instaces. Every meta type instance in this value index
/// MUST have a field named "Value" which has a type of NUM, LIST, or EQUATION
/// Otherwise, we can't get a value from the type and thus it shouldn't be placed in the value index.
pub struct ValueIndex<'a> {
    values: HashMap<String, MetaTypeInstance<'a>>,
}

impl ValueIndex<'_> {
    pub fn get_value<'a>(&'a self, meta_instance_name: &str, field_name: &str) -> &'a Value {
        todo!()
    }

    // Values can only be set for MetaTypeInstances which have NUMs as the type for the field
    // Returns error on a type mis-match
    pub fn set_value(&mut self, meta_instance_name: &str, field_name: &str, field_value: Value) {
        todo!()
    }

    // This adds a value to the meta type instace's Value field if the value field is a list
    // returns error on a type mis-match (assumes a list)
    pub fn add_value(&mut self, meta_instance_name: &str, field_name: &str, field_value: Value) {

    }
}

// pub struct DataIndex<'a> {
//     data: HashMap<String, MetaTypeInstance<'a>>,
//     modifiers: Vec<Modifier>
// }

// impl DataIndex<'_> {
//     pub fn get_value(&self, name: &String, used_for: Option<&String>) -> i32 {
//         if let Some(mti) = self.data.get(name) {
//             if let Some(use_case) = used_for {
//                 if let Some(m) = self.modifiers.iter().find(|modifier| modifier.target.eq(name)) {
//                     return MetaTypeInstance::get_value(self, mti).unwrap() + m.apply_modifier(self, use_case)
//                 }
//             }
//             return MetaTypeInstance::get_value(self, mti).unwrap()
//         }
//         return 0
//     }
// }

// pub struct Modifier {
//     name: String,
//     target: String,
//     val_name: String,
//     source: String,
//     use_case: ModifierUseCase
// }

// impl Modifier {
//     fn apply_modifier(&self, data: &DataIndex, use_case: &String) -> i32 {
//         match &self.use_case {
//             ModifierUseCase::Never => 0,
//             ModifierUseCase::Always => data.get_value(&self.val_name, Some(&self.name)),
//             ModifierUseCase::OnMatch(s) => if s.eq(use_case) {
//                 data.get_value(&self.val_name, Some(&self.name))
//             } else {
//                 0
//             }
//         }
//     }
// }

// pub enum ModifierUseCase {
//     Never,
//     Always,
//     OnMatch(String)
// }

// pub struct CharacterData<'a> {
//     types: TypeIndex,
//     data: DataIndex<'a>
// }

// impl CharacterData<'_> {

// }