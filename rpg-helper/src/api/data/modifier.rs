use crate::api::data::{context::Context, error::DataError, tag::{Tag, TagTemplate}};

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

/// A modifier conditionally applies a change to a numeric value
#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Modifier
{
    pub name: Tag,
    pub target: ModifierTarget,    // Target can be changed for a modifier (example, Flexible Formuliac Magic can increase or decrease specific formuliac spell's levels)
    // A condition is checked based on the type of modifier target.
    // If the target is a single target, then we simply check the condition
    // with the given tag directly.
    // However, if we are targeting something with a matching prefix / suffix,
    // then we will modifiy the condition with that tag matching the prefix or suffix
    // This allows modifiers to be checked conditionally for each value it modifies.
    pub condition: Tag,
    pub change: ModifierChange,
}

impl Modifier
{
    pub fn new(name: Tag, target: ModifierTarget, condition: Tag, change: ModifierChange) -> Modifier
    {
        Modifier { name, target, condition, change }
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum ModifierTarget
{
    Single(Tag),
    MatchingEnd(Tag),
    MatchingStart(Tag),
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
    all_modifiers: HashMap<Tag, Modifier>,          // Stores all the modifier data for this modifier set
    single_target_modifiers: HashMap<Tag, HashSet<Tag>>,     // Modifiers which target a single value. Input is target tag, output is the set of tags for modifiers
    conditional_modifiers: HashSet<Tag>,                // Modifiers which target based on a value's tag (See modifier target)
}

impl ModifierSet
{
    pub fn new() -> ModifierSet
    {
        ModifierSet
        { 
            all_modifiers: HashMap::new(),
            single_target_modifiers: HashMap::new(),
            conditional_modifiers: HashSet::new(),
        }
    }

    pub fn apply_modifiers(&self, dataset: &Context, t: &Tag, mut v: f32) -> Result<f32, DataError>
    {
        // First apply single target modifiers that we know will affect this tag
        if let Some(modifiers) = self.single_target_modifiers.get(t)
        {
            for modifier in modifiers
            {
                if let Some(modifier) = self.all_modifiers.get(modifier)
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

        // Now apply modifiers based on conditional prefix / suffix comparison and conditionals
        for modifier in self.conditional_modifiers.iter()
        {
            if let Some(modifier) = self.get_modifier(modifier)
            {
                // Check if suffix or prefix matches
                let conditional_tag = t.add_suffix(&modifier.condition);
                let apply_modifier = match &modifier.target
                {
                    ModifierTarget::Single(_) => return Err(DataError::InvalidState(format!("Expected modifier {} to be conditional prefix / suffix modifier. Was single target.", modifier.name.clone()))),
                    ModifierTarget::MatchingEnd(tag) => t.has_suffix(tag.to_str()),
                    ModifierTarget::MatchingStart(tag) => t.has_prefix(tag.to_str()),
                };

                if apply_modifier
                {
                    if dataset.eval_conditional(&conditional_tag)?
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
            }
            else
            {
                return Err(DataError::InvalidState(format!("Expected to have modifier {} in modifier set.", modifier.clone())));
            }
        }
        Ok(v)
    }

    /// Sets the value of a modifier for this modifier set.
    /// Conflicts of modifiers are resolved by the name of the modifier.
    /// If a matching modifier is found, the old value of the modifier
    /// is returned and the new modifier is used.
    pub fn set_modifier(&mut self, m: Modifier) -> Option<Modifier>
    {
        let m_name = m.name.clone();
        let m_target = m.target.clone();
        let old = self.all_modifiers.insert(m.name.clone(), m);

        // If we have some old modifier, we have some more logic to handle
        // We will remove all it's state tracking from the set so that we can set
        // things up properly
        if let Some(old_mod) = &old
        {
            match &old_mod.target
            {
                ModifierTarget::Single(target) =>
                {
                    if let Some(set) = self.single_target_modifiers.get_mut(target)
                    {
                        set.remove(&old_mod.name);
                    }
                },
                ModifierTarget::MatchingEnd(_) | ModifierTarget::MatchingStart(_) =>
                {
                    self.conditional_modifiers.remove(&old_mod.name);
                },
            }
        }

        // The modifier has been added to the "all modifiers" set, but
        // now we need to add it to the helper state tracking of the modifier set
        // based on what it targets.
        match &m_target
        {
            ModifierTarget::Single(target) =>
            {
                if !self.single_target_modifiers.contains_key(target)
                {
                    self.single_target_modifiers.insert(target.clone(), HashSet::new());
                }

                if let Some(set) = self.single_target_modifiers.get_mut(target)
                {
                    set.insert(m_name);
                }
            },
            ModifierTarget::MatchingEnd(_) | ModifierTarget::MatchingStart(_) =>
            {
                self.conditional_modifiers.insert(m_name);
            },
        }
        old
    }

    pub fn get_modifier(&self, modifier_name: &Tag) -> Option<&Modifier>
    {
        self.all_modifiers.get(modifier_name)
    }

    pub fn remove_modifier(&mut self, modifier_name: &Tag) -> Option<Modifier>
    {
        if let Some(old_mod) = self.all_modifiers.remove(modifier_name)
        {
            match &old_mod.target
            {
                ModifierTarget::Single(target) =>
                {
                    if let Some(set) = self.single_target_modifiers.get_mut(target)
                    {
                        set.remove(&old_mod.name);
                        if set.is_empty()
                        {
                            self.single_target_modifiers.remove(target);
                        }
                    }
                },
                ModifierTarget::MatchingEnd(_) | ModifierTarget::MatchingStart(_) =>
                {
                    self.conditional_modifiers.remove(&old_mod.name);
                },
            }
            Some(old_mod)
        }
        else
        {
            None
        }
    }

    pub fn has_modifier(&self, modifier_name: &Tag) -> bool
    {
        self.all_modifiers.contains_key(modifier_name)
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, Tag, Modifier>
    {
        self.all_modifiers.iter()
    }
}

impl IntoIterator for ModifierSet
{
    type Item = (Tag, Modifier);

    type IntoIter = std::collections::hash_map::IntoIter<Tag, Modifier>;

    fn into_iter(self) -> Self::IntoIter
    {
        self.all_modifiers.into_iter()
    }
}

pub struct ModifierTemplate
{
    pub name_template: TagTemplate,
    pub target_template: TagTemplate,
    pub condition: Tag,
    pub change: ModifierChange,
}