use std::{collections::HashMap, fmt::Display};

use crate::data::equation::Equation;

use super::ValueIndex;

#[derive(PartialEq)]
pub struct MetaType {
    type_name: String,
    fields: Vec<MetaField>
}

impl MetaType {
    pub fn new(type_name: String) -> MetaTypeBuilder {
        MetaTypeBuilder {
            type_name,
            fields: vec![]
        }
    }

    pub fn get_name(&self) -> &str {
        &self.type_name
    }

    // Might need to made pub, idk yet
    fn get_field(&self, field_name: &str) -> Option<&MetaField> {
        self.fields.iter().find(|f| f.field_name.eq(field_name))
    }
}

impl Display for MetaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Err(e) = write!(f, "{} {{\n", self.type_name) {
            return Err(e)
        }
        for mf in &self.fields {
            if let Err(e) = write!(f, "{}\n", mf) {
                return Err(e);
            }
        }
        write!(f, "}}")
    }
}

pub struct MetaTypeBuilder {
    type_name: String,
    fields: Vec<MetaField>
}

impl MetaTypeBuilder {
    pub fn define_field(&mut self, field_name: String, field_type: Type) {
        // TODO: Error handling for making sure that no field has the same name as another existing field
        self.fields.push(MetaField {field_name, field_type})
    }

    pub fn build(self) -> MetaType {
        MetaType {
            type_name: self.type_name,
            fields: self.fields
        }
    }
}

#[derive(PartialEq)]
struct MetaField {
    field_name: String,
    field_type: Type
}

impl Display for MetaField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:>10}:    {:?}", self.field_name, self.field_type)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Num,
    Text,
    List(Box<Type>),
    Enum(Vec<String>),
    // Meta(String), // Meta types can just be treated as equations
    Equation(Equation)
}

pub struct DataConversionErr;

#[derive(Clone, PartialEq)]
pub struct Value<'a> {
    t: Type,
    d: Data<'a>
}

impl Value<'_> {
    pub fn as_f32(&self, container: &MetaTypeInstance, data: &ValueIndex) -> Option<f32> {
        match &self.d {
            Data::Num(n) => Some(*n),
            Data::Text(_) => None,
            Data::List(l) => l.iter().fold(Some(0 as f32), |a: Option<f32>, v| {
                if let Some(a) = a {
                    if let Some(v) = v.as_f32(container, data) {
                        Some(a + v)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }),
            Data::Enum(_) => None,
            // Data::Meta(t) => if let Some(v) = t.get_field_value("Value") {
            //     v.as_f32(t, data)
            // } else {
            //     None
            // },
            Data::Equation(e) => if let Ok(v) = e.evaluate(container, data) {
                Some(v)
            } else {
                None
            }
        }
    }
}

#[derive(Clone, PartialEq)]
enum Data<'a> {
    Num(f32),
    Text(String),
    List(Vec<Value<'a>>),
    Enum(String),
    // Meta(MetaTypeInstance<'a>), // These are used for MetaTypeInstances that don't have a name (they are named by their field instead)
    Equation(&'a Equation)
}

#[derive(Clone, PartialEq)]
pub struct MetaTypeInstance<'a>  {
    // Name is implicit in anything that holds an instance
    t: &'a MetaType,
    fields: HashMap<String, Value<'a>>,
}

impl<'g> MetaTypeInstance<'g> {
    pub fn new<'a>(t: &'a MetaType) -> MetaTypeInstanceBuilder<'a> {
        MetaTypeInstanceBuilder {
            t,
            fields: HashMap::new(),
        }
    }

    pub fn get_field_value<'a>(&'a self, field_name: &str) -> Option<&Value<'a>> {
        self.fields.get(field_name)
    }
}


pub struct MetaTypeInstanceErr;

pub struct MetaTypeInstanceBuilder<'a> {
    t: &'a MetaType,
    fields: HashMap<String, Value<'a>>
}

impl<'a> MetaTypeInstanceBuilder<'a> {
    pub fn init_field(&'a mut self, field_name: String, field_value: Value<'a>) -> Result<(), MetaTypeInstanceErr> {
        if let Some(field) = self.t.get_field(&field_name) {
            if field_value.t == field.field_type {
                self.fields.insert(field_name, field_value);
                Ok(())
            } else {
                Err(MetaTypeInstanceErr) // Field Type Mis-match
            }
        } else {
            Err(MetaTypeInstanceErr) // Type does not have field with the given name
        }
    }
}