use std::{collections::HashMap, fmt::Display};

use crate::data::equation::Equation;
use crate::error::*;

use super::{TypeIndex, ValueIndex};

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

    pub fn get_field_type(&self, field_name: &str) -> Option<&Type> {
        if let Some(field) = self.fields.iter().find(|f| f.field_name.eq(field_name)) {
            Some(&field.field_type)
        } else {
            None
        }
    }

    // Might need to made pub, idk yet
    fn get_field(&self, field_name: &str) -> Option<&MetaField> {
        self.fields.iter().find(|f| f.field_name.eq(field_name))
    }

    pub fn get_fields(&self) -> Vec<String> {
        self.fields.iter().map(|f| f.field_name.clone()).collect()
    }

    pub fn get_default<'a>(&'a self, types: &'a TypeIndex) -> MetaTypeInstance<'a> {
        let mut result = MetaTypeInstance::new(&self);
        for f in &self.fields {
            result.init_field(f.field_name.to_string(), f.field_type.get_default(types));
        }
        result.build(types)
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

    pub fn has_field_defined(&self, field_name: &str) -> bool {
        self.fields.iter().any(|f| f.field_name.eq(field_name))
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
    Meta(String),
    Equation(Equation)
    // TODO: Add built-in type for die rolls. Defined by a string of the form: "1d10", "3d6", etc
    // Value is retrieved by an in-built roll or by input of what the roll result was
    // For this, we will need equations to support tertiary operators for
    // an equation that might have a die roll. I.E.
    //          Formuliac Casting Total = Casting Score + Stress Die
    //
    //          "Stress Die": {
    //              "Type": "1d10",
    //              "Exploding": [true, 1],
    //              "Botching": [true, 0]
    //          },
    //          "Simple Roll": {
    //              "Type": "1d10",
    //              "Exploding": [false]
    //              "Botching": [false]
    //          }
    // When an equation is evaluated, if it requires a Die roll, it will request produce the list of
    // Die rolls required (in order) and request either an input for a roll or have the option of rolling
    // through the application
}

impl Type {
    fn to_equation(&self) -> Option<&Equation> {
        if let Type::Equation(e) = &self {
            Some(e)
        } else {
            None
        }
    }

    pub fn get_default<'a>(&self, types: &'a TypeIndex) -> Value<'a> {
        match &self {
            Type::Num => Value::new_num(0.0f32),
            Type::Text => Value::new_text("".to_string()),
            Type::List(_) => Value::new_list(Vec::<Value>::new(), self.to_owned()).unwrap(),
            Type::Enum(variants) => Value::new_enum(variants.first().unwrap().clone(), self.to_owned()).unwrap(),
            Type::Meta(m) => Value::new_meta_instance(m.to_string(), types.get_type(m).unwrap().get_default(types)),
            Type::Equation(_) => Value::new_equation(self.to_owned()).unwrap(),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Type::Num => write!(f, "Num"),
            Type::Text => write!(f, "Text"),
            Type::List(t) => write!(f, "List<{}>", t),
            Type::Enum(e) => {
                write!(f, "Enum[")?;
                for s in e {
                    write!(f, "{}", s)?;
                }
                write!(f, "]")
            },
            Type::Equation(e) => write!(f, "Equation[{}]", e),
            Type::Meta(s) => write!(f, "Meta[{}]", s),
        }
    }
}



#[derive(Clone, PartialEq)]
pub struct Value<'a> {
    t: Type,
    d: Data<'a>
}

impl<'g> Value<'g> {
    pub fn new_num<'a>(num: f32) -> Value<'a> {
        Value {
            t: Type::Num,
            d: Data::Num(num)
        }
    }

    pub fn new_text<'a>(text: String) -> Value<'a> {
        Value {
            t: Type::Text,
            d: Data::Text(text)
        }
    }

    pub fn new_list<'a>(list: Vec<Value<'a>>, t: Type) -> Result<Value<'a>, DataConversionError> {
        if let Type::List(lt) = &t {
            for v in list.iter() {
                if &v.t != lt.as_ref() {
                    return Err(DataConversionError)
                }
            }
            Ok(Value {
                t,
                d: Data::List(list)
            })
        } else {
            Err(DataConversionError)
        }
    }
    
    pub fn new_enum<'a>(val: String, t: Type) -> Result<Value<'a>, DataConversionError> {
        if let Type::Enum(variants) = &t {
            if variants.contains(&val) {
                Ok(Value {
                    t,
                    d: Data::Enum(val)
                })
            } else {
                Err(DataConversionError)
            }
        } else {
            Err(DataConversionError)
        }
    }

    pub fn new_equation<'a>(t: Type) -> Result<Value<'a>, DataConversionError> {
        if let Type::Equation(_) = &t {
            Ok(Value {
                t,
                d: Data::Equation
            })
        } else {
            Err(DataConversionError)
        }
    }

    pub fn new_meta_instance<'a>(meta_type_name: String, inst: MetaTypeInstance<'a>) -> Value<'a> {
        Value {
            t: Type::Meta(meta_type_name),
            d: Data::Meta(inst)
        }
    }

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
            Data::Meta(t) => if let Some(v) = t.get_field_value("Value") {
                v.as_f32(t, data)
            } else {
                None
            },
            Data::Equation => if let Ok(v) = self.t.to_equation().unwrap().evaluate(container, data) {
                Some(v)
            } else {
                None
            }
        }
    }

    pub fn as_mut_f32(&mut self) -> Option<&mut f32> {
        match &mut self.d {
            Data::Num(n) => Some(n),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        todo!()
    }

    pub fn as_list(&self) -> Option<&Vec<Value>> {
        if let Data::List(l) = &self.d {
            Some(l)
        } else {
            None
        }
    }

    pub fn as_mut_list(&mut self) -> Option<&mut Vec<Value<'g>>> {
        if let Data::List(l) = &mut self.d {
            Some(l)
        } else {
            None
        }
    }

    pub fn get_type(&self) -> &Type {
        &self.t
    }
}

impl Display for Value<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.t, self.d)
    }
}

#[derive(Clone, PartialEq)]
enum Data<'a> {
    Num(f32),
    Text(String),
    List(Vec<Value<'a>>),
    Enum(String),
    Meta(MetaTypeInstance<'a>), // These are used for MetaTypeInstances that don't have a name (they are named by their field instead)
    Equation
}

impl Display for Data<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Data::Num(n) => write!(f, "{}", n),
            Data::Text(t) => write!(f, "{}", t),
            Data::List(l) => {
                write!(f, "[")?;
                for v in l {
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            },
            Data::Enum(s) => write!(f, "{}", s),
            Data::Equation=> Ok(()),
            Data::Meta(m) => write!(f, "{}", m),
        }
    }
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

    pub fn get_field_value(&self, field_name: &str) -> Option<&Value<'g>> {
        self.fields.get(field_name)
    }

    pub fn get_mut_field_value(&mut self, field_name: &str) -> Option<&mut Value<'g>> {
        self.fields.get_mut(field_name)
    }

    pub fn get_type(&self) -> &'g MetaType {
        &self.t
    }

    pub fn set_field_value(&mut self, field_name: &str, field_value: Value<'g>) -> Result<Option<Value<'g>>, InsertionError> {
        if let Some(v) = self.fields.get(field_name) {
            if v.t == field_value.t {
                Ok(self.fields.insert(field_name.to_string(), field_value))
            } else {
                Err(InsertionError) // Field types do not match
            }
        } else {
            Err(InsertionError) // Field does not exist
        }
    }
}

impl Display for MetaTypeInstance<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\n")?;
        for (s, v) in &self.fields {
            write!(f, "{:>10}: {}\n", s, v)?;
        }
        write!(f, "}}\n")?;
        Ok(())
    }
}



pub struct MetaTypeInstanceBuilder<'a> {
    t: &'a MetaType,
    fields: HashMap<String, Value<'a>>
}

impl<'a> MetaTypeInstanceBuilder<'a> {
    pub fn init_field(&mut self, field_name: String, field_value: Value<'a>) -> Result<(), FieldError> {
        if let Some(field) = self.t.get_field(&field_name) {
            if field_value.t == field.field_type {
                self.fields.insert(field_name, field_value);
                Ok(())
            } else {
                Err(FieldError) // Field Type Mis-match
            }
        } else {
            Err(FieldError) // Type does not have field with the given name
        }
    }

    // Returns a list of the needed fields to build this instance
    pub fn get_needed_fields(&self) -> Vec<(String, Type)> {
        let mut result = vec![];
        let fields = self.t.get_fields();
        for field in &fields {
            if !self.fields.contains_key(field) {
                result.push((field.clone(), self.t.get_field_type(field).unwrap().clone()));
            }
        }
        result
    }

    pub fn build(mut self, types: &'a TypeIndex) -> MetaTypeInstance<'a> {
        // TODO: Initialize un-initialized fields to proper values
        let needed = self.get_needed_fields();
        for (s, t) in needed {
            self.fields.insert(s, t.get_default(types));
        }
        MetaTypeInstance {
            t: self.t,
            fields: self.fields,
        }
    }
}