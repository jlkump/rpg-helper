use std::{collections::HashMap, fmt::Display};

use crate::data::equation::Equation;
use crate::error::*;

use super::indexes::type_index::TypeIndex;
use super::indexes::value_index::ValueIndex;


#[derive(PartialEq, Debug)]
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

    pub fn get_type_name(&self) -> &str {
        &self.type_name
    }

    pub fn get_field_type(&self, field_name: &str) -> Option<&Type> {
        if let Some(field) = self.fields.iter().find(|f| f.field_name.eq(field_name)) {
            Some(&field.field_type)
        } else {
            None
        }
    }
    
    pub fn get_fields(&self) -> Vec<String> {
        self.fields.iter().map(|f| f.field_name.clone()).collect()
    }
    
    pub fn get_default<'a>(&'a self, types: &'a TypeIndex) -> MetaTypeInstance<'a> {
        let mut result = MetaTypeInstance::new(&self);
        for f in &self.fields {
            result = result.init_field(f.field_name.to_string(), f.field_type.get_default(types)).unwrap();
        }
        result.build(types)
    }

    // Helper method
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
    pub fn define_field(mut self, field_name: String, field_type: Type) -> Result<Self, DefinitionError<String>> {
        if self.has_field_defined(&field_name) {
            Err(DefinitionError::Redef(field_name))
        } else {
            self.fields.push(MetaField {field_name, field_type});
            Ok(self)
        }
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

#[derive(Debug, PartialEq)]
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
    pub fn init_field(mut self, field_name: String, mut field_value: Value<'a>) -> Result<Self, FieldError> {
        if let Some(field) = self.t.get_field(&field_name) {
            if field_value.t == field.field_type {
                self.fields.insert(field_name, field_value);
                Ok(self)
            } else {
                Err(FieldError::Mismatch(field_value.t.to_string(), field.field_type.to_string())) // Field Type Mis-match
            }
        } else {
            Err(FieldError::Nonexistant(field_name)) // Type does not have field with the given name
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

#[derive(Clone, Debug, PartialEq)]
pub struct Value<'a> {
    t: Type,
    d: Data<'a>
}

impl<'g> Value<'g> {
    pub fn new_num<'a>(num: f32) -> Value<'a> {
        Value {
            t: Type::Num,
            d: Data::Num(num),
        }
    }

    pub fn new_text<'a>(text: String) -> Value<'a> {
        Value {
            t: Type::Text,
            d: Data::Text(text),
        }
    }

    pub fn new_list<'a>(list: Vec<Value<'a>>, t: Type) -> Result<Value<'a>, FieldError> {
        if let Type::List(lt) = &t {
            for v in list.iter() {
                if &v.t != lt.as_ref() {
                    return Err(FieldError::Mismatch(v.t.to_string(), lt.as_ref().to_string()))
                }
            }
            Ok(Value {
                t,
                d: Data::List(list),
            })
        } else {
            Err(FieldError::Mismatch("List<?>".to_owned(), t.to_string()))
        }
    }
    
    pub fn new_enum<'a>(val: String, t: Type) -> Result<Value<'a>, FieldError> {
        if let Type::Enum(variants) = &t {
            if variants.contains(&val) {
                Ok(Value {
                    t,
                    d: Data::Enum(val),
                })
            } else {
                Err(FieldError::Nonexistant(t.to_string()))
            }
        } else {
            Err(FieldError::Mismatch(t.to_string(), val))
        }
    }

    pub fn new_equation<'a>(t: Type) -> Result<Value<'a>, FieldError> {
        if let Type::Equation(_) = &t {
            Ok(Value {
                t,
                d: Data::Equation,
            })
        } else {
            Err(FieldError::Mismatch("Equation".to_string(), t.to_string()))
        }
    }

    pub fn new_meta_instance<'a>(meta_type_name: String, inst: MetaTypeInstance<'a>) -> Value<'a> {
        Value {
            t: Type::Meta(meta_type_name),
            d: Data::Meta(inst),
        }
    }

    pub fn new_meta_ref<'a>(meta_ref_name: String, t: Type) -> Value<'a> {
        Value {
            t,
            d: Data::MetaRef(meta_ref_name),
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
            Data::MetaRef(r) => {
                let mut result = None;
                // First try to see if the reference is to a value contained in self
                if let Some(v) = container.get_field_value(r) {
                    result = v.as_f32(container, data);
                }
                // If the value is not contained in self, see if it is contained in the data
                if result.is_none() {
                    if let Some(m) = data.get_instance(r) {
                        if let Some(v) = m.get_field_value("Value") {
                            result = v.as_f32(m, data)
                        }
                    }
                }
                result
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

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Num,
    Text,
    List(Box<Type>),
    Enum(Vec<String>),
    Meta(String), // Name of the meta type
    Equation(Equation),   // Equation is owned by the type and thus not named
    // EquationRef(String),  // Refrence to an equation by name
    MetaRef(String) // MetaRef has the name of the meta type, just like Meta
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
            Type::MetaRef(_) => panic!("Meta ref not given. Can not initialize a default val"),
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
            Type::MetaRef(r) => write!(f, "MetaRef[{}]", r),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Data<'a> {
    Num(f32),
    Text(String),
    List(Vec<Value<'a>>),
    Enum(String),
    Meta(MetaTypeInstance<'a>), // The meta type is accessed by the field name
    Equation,
    // EquationRef,
    MetaRef(String) // Name of the actual reference to the meta instance
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
            Data::MetaRef(r) => write!(f, "{}", r),
        }
    }
}