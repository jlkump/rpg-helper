use crate::api::data::{context::Context, error::{DataError, TemplateError}, tag::{Tag, TagTemplate}, template::{Template, Templated}};

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
    name_template: Templated<TagTemplate, Tag>,
    target_template: Templated<ModifierTargetTemplate, ModifierTarget>,
    condition_template: Templated<TagTemplate, Tag>,
    change_template: Templated<ModifierChangeTemplate, ModifierChange>,
}

impl ModifierTemplate
{
    pub fn new() -> Self
    {
        todo!()
    }
}

impl Template<Modifier> for ModifierTemplate
{
    fn get_required_inputs(&self) -> HashSet<String>
    {
        let mut result = self.name_template.get_required_inputs();
        result.extend(self.target_template.get_required_inputs());
        result.extend(self.condition_template.get_required_inputs());
        result.extend(self.change_template.get_required_inputs());
        result
    }

    /// Inserts given tag value into all matching template inputs
    fn insert_template_value(&mut self, input_name: &str, input_value: &Tag) -> Option<Modifier>
    {
        self.name_template.insert_template_value(input_name, input_value);
        self.target_template.insert_template_value(input_name, input_value);
        self.condition_template.insert_template_value(input_name, input_value);
        self.change_template.insert_template_value(input_name, input_value);

        match (&self.name_template, &self.target_template, &self.condition_template, &self.change_template)
        {
            (
                Templated::Complete(name),
                Templated::Complete(target),
                Templated::Complete(condition),
                Templated::Complete(change)
            ) => Some(Modifier {
                name: name.clone(),
                target: target.clone(),
                condition: condition.clone(),
                change: change.clone(),
            }),
            _ => None,
        }
    }
    
    fn attempt_complete(&self) -> Result<Modifier, super::error::TemplateError>
    {
        match (self.name_template.as_complete(), self.target_template.as_complete(), self.condition_template.as_complete(), self.change_template.as_complete())
        {
            (
                Some(name),
                Some(target),
                Some(condition),
                Some(change),
            ) => Ok(Modifier
            {
                name: name.clone(),
                target: target.clone(),
                condition: condition.clone(),
                change: change.clone(),
            }),
            _ => Err(TemplateError::MissingTemplateValues(self.get_required_inputs().into_iter().collect()))
        }
    }

    
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
enum ModifierTargetTemplate
{
    Single(TagTemplate),
    MatchingEnd(TagTemplate),
    MatchingStart(TagTemplate),
}

impl Template<ModifierTarget> for ModifierTargetTemplate
{
    fn get_required_inputs(&self) -> HashSet<String>
    {
        match self
        {
            ModifierTargetTemplate::Single(tag_template) | ModifierTargetTemplate::MatchingEnd(tag_template) | ModifierTargetTemplate::MatchingStart(tag_template) => tag_template.get_required_inputs(),
        }
    }

    fn insert_template_value(&mut self, input_name: &str, input_value: &Tag) -> Option<ModifierTarget>
    {
        // A closure to make the mapping from the template to true value easier and cleaner
        // Takes the value of self and outputs the tag template inner value combined with a fn that converts
        // a tag to a ModifierTarget (which is simply the enum without the parentheses [Very cool, I did not know that was possible]).
        let (tag_template, wrapper): (&mut TagTemplate, fn(Tag) -> ModifierTarget) = match self
        {
            ModifierTargetTemplate::Single(t) => (t, ModifierTarget::Single),
            ModifierTargetTemplate::MatchingEnd(t) => (t, ModifierTarget::MatchingEnd),
            ModifierTargetTemplate::MatchingStart(t) => (t, ModifierTarget::MatchingStart),
        };
        tag_template.insert_template_value(input_name, input_value).map(wrapper)
    }

    fn attempt_complete(&self) -> Result<ModifierTarget, super::error::TemplateError>
    {
        let (tag_template, wrapper): (&TagTemplate, fn(Tag) -> ModifierTarget) = match self
        {
            ModifierTargetTemplate::Single(t) => (t, ModifierTarget::Single),
            ModifierTargetTemplate::MatchingEnd(t) => (t, ModifierTarget::MatchingEnd),
            ModifierTargetTemplate::MatchingStart(t) => (t, ModifierTarget::MatchingStart),
        };
        tag_template.attempt_complete().map(wrapper)
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
enum ModifierChangeTemplate
{
    BasicValue(f32),
    FromOtherValue(TagTemplate),
}

impl Template<ModifierChange> for ModifierChangeTemplate
{
    fn get_required_inputs(&self) -> HashSet<String>
    {
        match self
        {
            ModifierChangeTemplate::BasicValue(_) => HashSet::new(),
            ModifierChangeTemplate::FromOtherValue(tag_template) => tag_template.get_required_inputs(),
        }
    }

    fn insert_template_value(&mut self, input_name: &str, input_value: &Tag) -> Option<ModifierChange>
    {
        match self
        {
            ModifierChangeTemplate::BasicValue(v) => Some(ModifierChange::BasicValue(*v)),
            ModifierChangeTemplate::FromOtherValue(tag_template) =>
            if let Some(tag) = tag_template.insert_template_value(input_name, input_value)
            {
                Some(ModifierChange::FromOtherValue(tag))
            }
            else
            {
                None
            },
        }
    }

    fn attempt_complete(&self) -> Result<ModifierChange, super::error::TemplateError>
    {
        match self
        {
            ModifierChangeTemplate::BasicValue(v) => Ok(ModifierChange::BasicValue(*v)),
            ModifierChangeTemplate::FromOtherValue(tag_template) =>
            match tag_template.attempt_complete()
            {
                Ok(tag) => Ok(ModifierChange::FromOtherValue(tag)),
                Err(e) => Err(e),
            },
        }
    }
}