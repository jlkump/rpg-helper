use std::{collections::HashMap, fmt::Display};

use crate::data::equation::Equation;

use super::{CharacterData, DataIndex, TypeIndex};

pub struct MetaType {
    type_name: String,
    fields: Vec<MetaField>
}

impl MetaType {
    pub fn new(type_name: String, fields: Vec<MetaField>) -> MetaType {
        MetaType {
            type_name,
            fields
        }
    }

    pub fn define_field(field_name: String, field_type: Type) -> MetaField {
        MetaField {
            field_name,
            field_type
        }
    }

    pub fn has_field(&self, field_name: &str) -> bool {
        self.fields.iter().any(|f| f.field_name.eq(field_name))
    }

    pub fn get_name(&self) -> String {
        self.type_name.clone()
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

pub struct MetaField {
    field_name: String,
    field_type: Type
}

impl Display for MetaField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:>10}:    {:?}", self.field_name, self.field_type)
    }
}

#[derive(Debug)]
pub enum Type {
    Int,
    String,
    List(Box<Type>),
    Enum(Vec<String>),
    Meta(String),
    Equation(Equation)
}

#[derive(Clone)]
pub enum FieldValue<'a> {
    Int(i32),
    String(String),
    List(Vec<FieldValue<'a>>),
    Enum(String),
    Meta(MetaTypeInstance<'a>),
    Equation(&'a Equation)
}

impl FieldValue<'_> {
    pub fn get_value(&self, data: &DataIndex, owner: &MetaTypeInstance) -> Option<i32> {
        match &self {
            FieldValue::Int(i) => Some(*i),
            FieldValue::Enum(_) => None,
            FieldValue::List(l) => l.iter().map(|f| {f.get_value(&data, owner)}).sum(),
            FieldValue::Meta(m) => MetaTypeInstance::get_value(&data, &m),
            FieldValue::String(_) => None,
            FieldValue::Equation(e) => Some(e.evaluate(&owner, data).unwrap() as i32)
        }
    }
}

#[derive(Clone)]
pub struct MetaTypeInstance<'a>  {
    name: String,
    t: &'a MetaType,
    fields: HashMap<String, FieldValue<'a>>,
}

impl<'g> MetaTypeInstance<'g> {
    pub fn new<'a>(name: String, t: &'a MetaType, index: &'a TypeIndex) -> MetaTypeInstance<'a> {
        let mut fields = HashMap::new();
        for f in &t.fields {
            fields.insert(f.field_name.clone(), Self::type_to_field(&f.field_type, index));
        }
        MetaTypeInstance {
            name,
            t,
            fields,
        }
    }

    fn type_to_field<'a>(t: &'a Type, index: &'a TypeIndex) -> FieldValue<'a> {
        match t {
            Type::Meta(s) => FieldValue::Meta(Self::new(s.clone(), index.get_type(s).expect("No field found"), index)),
            Type::Int => FieldValue::Int(0),
            Type::List(_) => todo!(),
            Type::String => FieldValue::String("".to_owned()),
            Type::Enum(vals) => FieldValue::Enum(vals[0].clone()),
            Type::Equation(e) => FieldValue::Equation(&e)
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_type(&self) -> &MetaType {
        &self.t
    }

    pub fn set_field(&mut self, field_name: &String, new_value: FieldValue<'g>) -> Option<FieldValue> {
        if self.fields.contains_key(field_name) {
            self.fields.insert(field_name.clone(), new_value)
        } else {
            None
        }
    }

    pub fn get_field(&self, field_name: &String) -> Option<FieldValue> {
        if let Some(field) = self.fields.get(field_name) {
            return Some(field.clone())
        }
        return None
    }

    pub fn get_value(data: &DataIndex, mti: &MetaTypeInstance) -> Option<i32> {
        if let Some(f) = mti.get_field(&"Value".to_owned()) {
            return f.get_value(data, mti)
        }
        return None
    }
}