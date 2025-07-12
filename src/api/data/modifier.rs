use crate::api::data::{error::DataError, tag::Tag, context::Context};

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

/// A modifier conditionally applies a change to a numeric value
#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Modifier
{
    pub name: Tag,
    pub target: Tag,    // Target can be changed for a modifier (example, Flexible Formuliac Magic can increase or decrease specific formuliac spell's levels)
    pub condition: Tag,
    pub change: ModifierChange,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum ModifierChange
{
    BasicValue(f32),
    FromOtherValue(Tag),
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct ModifierSet
{
    modifiers: HashMap<Tag, Modifier>,
    target_to_modifiers: HashMap<Tag, HashSet<Tag>>,
}

impl ModifierSet
{
    pub fn new() -> ModifierSet
    {
        ModifierSet { 
            modifiers: HashMap::new(), 
            target_to_modifiers: HashMap::new() 
        }
    }

    pub fn apply_modifiers(&self, dataset: &Context, t: &Tag, mut v: f32) -> Result<f32, DataError>
    {
        if let Some(modifiers) = self.target_to_modifiers.get(t)
        {
            for modifier in modifiers
            {
                if let Some(modifier) =  self.modifiers.get(modifier)
                {
                    if dataset.eval_conditional(&modifier.condition)?
                    {
                        match &modifier.change
                        {
                            ModifierChange::BasicValue(add) => v += add,
                            ModifierChange::FromOtherValue(tag) => 
                            {
                                if let Some(add) = dataset.get_value(tag)?
                                {
                                    v += add;
                                } 
                                else
                                {
                                    return Err(DataError::value_dne(tag.clone()))
                                }
                            },
                        }
                    }
                }
                else
                {
                    return Err(DataError::InvalidState(format!("Expected to have modifier {} in modifier set.", modifier.clone())));
                }
            }
        }
        Ok(v)
    }

    pub fn add_modifier(&mut self, m: Modifier)
    {
        // Can not stack modifiers (currently)
        if !self.modifiers.contains_key(&m.name)
        {
            let mut v = HashSet::new();
            if let Some(m) = self.target_to_modifiers.get(&m.target)
            {
                v = m.clone();
            }
            v.insert(m.name.clone());
            self.target_to_modifiers.insert(m.target.clone(), v);
            self.modifiers.insert(m.name.clone(), m);
        }
    }

    pub fn get_modifier(&self, modifier_name: &Tag) -> Option<&Modifier>
    {
        self.modifiers.get(modifier_name)
    }

    pub fn remove_modifier(&mut self, modifier_name: &Tag) -> Option<Modifier>
    {
        if let Some(m) = self.modifiers.remove(modifier_name)
        {
            if let Some(v) = self.target_to_modifiers.get_mut(&m.target)
            {
                v.remove(&m.name);
            }
            Some(m)
        }
        else
        {
            None
        }
    }

    pub fn has_modifier(&self, modifier_name: &Tag) -> bool
    {
        self.modifiers.contains_key(modifier_name)
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, Tag, Modifier>
    {
        self.modifiers.iter()
    }
}