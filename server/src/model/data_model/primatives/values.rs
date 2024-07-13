use serde::{Deserialize, Serialize};

use boolean::Bool;
use die_roll::DieRoll;
use enumeration::Enumeration;
use list::List;
use meta::MetaInst;
use number::Number;

use crate::model::data_model::storage::{types::{BooleanTypeRef, EquationRef, NumberTypeRef, TypeRef}, values::{MetaInstRef, ValueRef}, view_context::ViewContext, ContainerKind, IndexRef, Query, QueryError, Storable};

use super::types::{boolean::BooleanType, number::NumberType};

pub mod boolean;
pub mod die_roll;
pub mod enumeration;
pub mod list;
pub mod meta;
pub mod number;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum Value {
    Num(Number),
    Bool(Bool),
    List(List),
    Enum(Enumeration),
    Meta(MetaInst),
    Equation(EquationRef),
    DieRoll(DieRoll), 
    MetaRef(MetaInstRef),
}

impl Storable for Value {
    fn get_container(&self) -> &ContainerKind {
        todo!()
    }
}

impl Value {
    pub fn get_type(&self) -> &TypeRef {
        match self {
            Value::Num(_) => todo!(),
            Value::Bool(_) => todo!(),
            Value::List(l) => todo!(),
            Value::Enum(e) => todo!(),
            Value::Meta(m) => todo!(),
            Value::Equation(_) => todo!(),
            Value::DieRoll(_) => todo!(),
            Value::MetaRef(_) => todo!(),
        }
    }

    pub fn as_number(&self, context: &ViewContext) -> Query<f32> {
        match self {
            Value::Num(n) => return Ok(n.value as f32),
            Value::List(list) => {
                // For lists, sum the value of the entries to get the number
                return Ok(list.iter().fold(0f32, |acc: f32, x| {
                    if let Ok(v) = x.as_number(context) {
                        acc + v
                    } else {
                        acc
                    }
                }));
            },
            Value::Meta(m) => {
                match m.get_value() {
                    Ok(v) => return v.as_number(context),
                    Err(e) => return Err(e),
                }
            },
            Value::Equation(e) => todo!(),
            Value::DieRoll(_) => todo!(),
            Value::MetaRef(_) => todo!(),
            _ => {}
        }
        return Err(QueryError::TypeMismatch(
            self.get_type().clone(), 
            context.value_to_ref::<NumberType, NumberTypeRef>(NumberType::generic())?.into()
        ));
    }

    pub fn as_bool(&self, context: &ViewContext) -> Query<bool> {
        match self {
            Value::Bool(b) => return Ok(b.value),
            Value::Meta(m) => {
                if let Ok(v) = m.get_value() {
                    return v.as_bool(context);
                }
            },
            Value::Equation(e) => {
                if let Ok(e) = e.to_ref(context) {
                    e.eval(None);
                }
            }
            Value::MetaRef(m) => {
                if let Ok(m) = m.to_ref(context) {
                    if let Ok(v) = m.get_value() {
                        return v.as_bool(context);
                    }
                }
            }
            _ => {},
        }
        Err(QueryError::TypeMismatch(
            self.get_type().clone(), 
            context.value_to_ref::<BooleanType, BooleanTypeRef>(BooleanType::generic())?.into()
        ))
    }

    pub fn get_name(&self) -> &str {
        todo!()
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum ValueEffect {
    AddToNum(f32),
    ChangeBool(bool),
    AddToList(Value),
    RemoveFromList(String),
    ChangeEnumValue(usize),
    ChangeMetaInstField(String, Box<ValueEffect>),
    ChangeEquationRef(EquationRef),
    ChangeDieRoll(DieRoll),
    ChangeMetaRef(MetaInstRef),
}

impl ValueEffect {
    pub fn get_mod_type(&self) -> &TypeRef {
        match self {
            ValueEffect::AddToNum(_) => todo!(),
            ValueEffect::ChangeBool(_) => todo!(),
            ValueEffect::AddToList(_) => todo!(),
            ValueEffect::RemoveFromList(_) => todo!(),
            ValueEffect::ChangeEnumValue(_) => todo!(),
            ValueEffect::ChangeMetaInstField(_, _) => todo!(),
            ValueEffect::ChangeEquationRef(_) => todo!(),
            ValueEffect::ChangeDieRoll(_) => todo!(),
            ValueEffect::ChangeMetaRef(_) => todo!(),
        }
    }
}

impl ValueEffect {
    /// Applys the effect to the given value
    /// If the effect is able to take place,
    /// the new value is returned. Otherwise,
    /// the old value is returned as is.
    pub fn apply(self, mut v: Value, context: &ViewContext) -> Value {
        match self {
            ValueEffect::AddToNum(num) => {
                if let Value::Num(other) = &mut v {
                    other.value = num + other.value;
                }
            },
            ValueEffect::ChangeBool(b) => {
                if let Value::Bool(other) = &mut v {
                    other.value = b;
                }
            },
            ValueEffect::AddToList(val) => {
                if let Value::List(list) = &mut v {
                    list.push(val);
                }
            },
            ValueEffect::RemoveFromList(target) => {
                if let Value::List(list) = &mut v {
                    // TODO: find the element to remove by name
                }
            },
            ValueEffect::ChangeEnumValue(inst) => {
                // Range checking to prevent the enum from being invalid
                if let Value::Enum(e) = &mut v {
                    let max = {
                        if let Ok(e) = e.t.to_ref(context) {
                            (e.types.len() - 1).max(0)
                        } else {
                            1
                        }
                    };
                    e.inst = inst.clamp(0, max);
                }
            },
            ValueEffect::ChangeMetaInstField(field, effect) => {
                if let Value::Meta(m) = &mut v {
                    if let Some(f) = m.fields.get(&field) {
                        if f.get_type() == effect.get_mod_type() {
                            m.fields.insert(field, effect.apply(f.clone(), context));
                        }
                    }
                }
            },
            ValueEffect::ChangeEquationRef(e) => {
                if let Value::Equation(_) = &mut v {
                    v = Value::Equation(e);
                }
            },
            ValueEffect::ChangeDieRoll(_) => todo!(),
            ValueEffect::ChangeMetaRef(_) => todo!(),
        }
        v
    }
}