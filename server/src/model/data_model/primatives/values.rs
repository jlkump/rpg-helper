use serde::{Deserialize, Serialize};

use boolean::Bool;
use die_roll::DieRoll;
use enumeration::Enumeration;
use list::List;
use meta::MetaInst;
use number::Number;

use crate::model::data_model::{get_game, storage::{types::{EquationRef, TypeRef}, values::MetaInstRef, IndexRef}};

use super::types::Type;

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
    pub fn apply(self, mut v: Value) -> Value {
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
                        if let Ok(e) = e.t.to_ref(get_game()) {
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
                            m.fields.insert(field, effect.apply(f.clone()));
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