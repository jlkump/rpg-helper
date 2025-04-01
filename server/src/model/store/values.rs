use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::model::{core::StoreContext, storable::values::Value};

pub struct ValueStore<'a> 
{
    values: HashMap<String, Value>,
    view_context: Option<&'a StoreContext>,
}


#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Clone)]
enum ValueRefSubtarget {
    ListElem(usize, Option<Box<ValueRefSubtarget>>),
    Field(ValueRef),
}

fn value_ref_subtarget_name_helper(res: &mut String, v_ref: &ValueRefSubtarget) {
    match v_ref {
        ValueRefSubtarget::ListElem(index, subtarget) => {
            res.push_str(&format!("[{}]", index));
            if let Some(sub) = subtarget {
                value_ref_subtarget_name_helper(res, sub)
            }
        },
        ValueRefSubtarget::Field(f) => {
            res.push('.');
            res.push_str(&f.get_ref_name());
        },
    }
}

impl IndexRef<Value> for ValueRef {
    fn get_container(&self) -> &super::ContainerKind {
        todo!()
    }
    
    fn get_ref_name(&self) -> String {
        let mut res = self.name.clone();
        match &self.subtarget {
            Some(v_ref) => {
                value_ref_subtarget_name_helper(&mut res, v_ref)
            },
            None => {},
        }
        res
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct MetaInstRef { // MetaRef could also be MetaInst
    // Hold data on the ruleset / setting it came from?
    pub type_name: String,
    pub ref_name: String,
}

impl IndexRef<MetaInst> for MetaInstRef {
    fn get_ref_name(&self) -> String {
        todo!()
    }
    
    fn get_container(&self) -> &super::ContainerKind {
        todo!()
    }
}

pub struct ModifierRef {
    name: String,
    target: ValueRef,
    ref_target: RefTarget,
}

impl IndexRef<Modifier> for ModifierRef {
    fn get_ref_name(&self) -> String {
        self.target.get_ref_name()
    }

    fn get_container(&self) -> &super::ContainerKind {
        todo!()
    }
}

impl IndexStorage<Modifier, ModifierRef> for ValueIndex<'_> {
    fn get<'a>(&'a self, r: &ModifierRef) -> super::Query<&'a Modifier> {
        match self.modifiers.get(&r.target) {
            Some(m) => {
                if let Some(context) = &self.view_context {
                    match m.iter().find(|v| {
                        match v.get_type(context) {
                            Ok(m) => m.name.eq(&r.name),
                            Err(_) => false,
                        }
                    }) {
                            Some(v) => Ok(v),
                            None => Err(r.to_dne_error()),
                    }
                } else {
                    Err(QueryError::ViewContextDoesNotExist)
                }
            },
            None => Err(r.to_dne_error()),
        }
    }
}