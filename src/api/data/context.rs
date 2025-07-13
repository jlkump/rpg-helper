use std::{collections::HashMap, mem::swap};

use crate::api::data::{attribute::AttributeSet, conditional::{Conditional, ConditionalSet}, effect::Effect, equation::{Equation, EquationSet}, error::{ConflictError, DataError}, modifier::{Modifier, ModifierSet}, tag::{Tag, TagSet}, DataType};

use serde::{Deserialize, Serialize};

/// A Context is the abstraction layer for interaction with the data-layer
/// of the application. It handles the evaluation of numeric values based
/// on modifiers, conditionals, and equations.
/// 
/// A modifier applies a change to an attribute or equation in the dataset
/// by addiing another value conditionally.
///
/// A conditional evaluates an AST using the tags and attribute values
/// of the dataset. It will return either true or false.
/// 
/// An equation acts like an attribute but depends upon the value of
/// other attributes in the dataset. For example, in Ars Magica, the
/// value of "Ability.Magic Theory" depends on the value of "Ability.Magic Theory.Exp"
#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Context
{
    general_tags: TagSet,
    state_tags: TagSet,
    atrs: AttributeSet,
    modifiers: ModifierSet,
    equations: EquationSet,
    conditionals: ConditionalSet,
    text_data: HashMap<Tag, String>,

    temporary_layers: Option<HashMap<LayerHandle, Context>>,
}

pub type LayerHandle = u32;

impl Context
{
    pub fn new() -> Context
    {
        Context { 
            general_tags: TagSet::new(), 
            state_tags: TagSet::new(),          // State tags are only modified by effects (or changed by a context layer)
            atrs: AttributeSet::new(), 
            modifiers: ModifierSet::new(),
            equations: EquationSet::new(),
            conditionals: ConditionalSet::new(),
            text_data: HashMap::new(),
            temporary_layers: None,
        }
    }

    pub fn has_tag(&self, t: &Tag) -> bool
    {
        self.general_tags.has_tag(t) || self.state_tags.has_tag(t)
    }

    pub fn has_attribute(&self, attribute_name: &Tag) -> bool
    {
        self.atrs.has_attribute(attribute_name)
    }

    pub fn has_modifier(&self, modifier_name: &Tag) -> bool
    {
        self.modifiers.has_modifier(modifier_name)
    }

    pub fn has_equation(&self, equation_name: &Tag) -> bool
    {
        self.equations.has_equation(equation_name)
    }
    
    pub fn has_conditional(&self, conditional_name: &Tag) -> bool
    {
        self.conditionals.has_conditional(conditional_name)
    }

    pub fn has_value(&self, value_name: &Tag) -> bool
    {
        self.has_attribute(value_name) || self.has_equation(value_name)
    }

    pub fn has_text(&self, text_name: &Tag) -> bool
    {
        self.text_data.contains_key(text_name)
    }

    /// Creates a new context that combines this context plus another context.
    /// If there are conflicting keys, the values of "other" are perfered
    /// over self
    pub fn layer_context(&mut self, other: &Self) -> Result<(), DataError>
    {
        // Insert all rhs values (overriding our own in the case of conflict)
        // Attributes
        for (tag, atr) in other.atrs.iter()
        {
            self.set_attribute(tag, atr.get_value())?;
        }

        // Modifiers
        for  (_, modifier) in other.modifiers.iter()
        {
            self.set_modifier(modifier.clone())?;
        }

        // Equations
        for  (_, equation) in other.equations.iter()
        {
            self.set_equation(equation.clone())?;
        }

        // Conditionals
        for  (_, conditional) in other.conditionals.iter()
        {
            self.set_conditional(conditional.clone())?;
        }

        // Text data
        for  (tag, text) in other.text_data.iter()
        {
            self.set_text_data(tag, text.clone())?;
        }

        // State tags
        self.state_tags = self.state_tags.layer(&other.state_tags);

        Ok(())
    }

    /// Introduced because there are cases where we want to temporarily
    /// layer the values of another ctx for effect application,
    /// then remove those layers after effect application.
    /// Temporary layers can not be modified, they only can overlay this
    /// ctx's values. Layering temporarily should be used carefully, as it can
    /// cause state where a value can not change if it overriden with a temporary layer.
    /// 
    /// Whenever possible, prefer `layer_context()` as it is less prone to logic errors
    pub fn layer_temporary_context(&mut self, other: Self) -> Result<LayerHandle, DataError>
    {
        todo!()
    }

    pub fn remove_temporary_context(&mut self, handle: LayerHandle) -> Result<Self, DataError>
    {
        todo!()
    }

    pub fn remove_all_temporary_contexts(&mut self) -> Vec<Self>
    {
        let mut result = vec![];
        let mut v = None;
        swap(&mut v, &mut self.temporary_layers);
        if let Some(v) = v
        {
            for (_, c) in v
            {
                result.push(c);
            }
            self.temporary_layers = None;
        }
        result
    }

    /// Modifies a given dataset according to the effect.
    /// Returns the modified dataset or an error if the modification failed.
    pub fn apply_effect(&mut self, e: &Effect) -> Result<(), DataError>
    {
        match e
        {
            Effect::AddStateTag(tag) => self.state_tags.add_tag(tag),
            Effect::RemoveStateTag(tag) => self.state_tags.remove_tag(tag),
            Effect::SetAttribute(tag, nv) => { self.set_attribute(tag, *nv)?; },
            Effect::SetEquation(equation) => { self.set_equation(equation.clone())?; },
            Effect::SetConditional(conditional) => { self.set_conditional(conditional.clone())?; },
            Effect::SetModifier(modifier) => { self.set_modifier(modifier.clone())?; },
            Effect::SetTextData(tag, text) => { self.set_text_data(tag, text.clone())?; }
            Effect::SetAttributeFromValue(tag, val) => { 
                if let Some(val) = self.get_value(val)?
                {
                    self.set_attribute(tag, val)?; 
                }
            },
        }
        Ok(())
    }

    /// Gets the value of an attribute (including equation aliases) 
    /// accounting for all modifiers applied.
    /// 
    /// Can error from modifier or equation evaluation failures
    pub fn get_value(&self, t: &Tag) -> Result<Option<f32>, DataError>
    {
        if let Some(a) = self.atrs.get(t)
        { 
            Ok(Some(self.modifiers.apply_modifiers(self, t, a.get_value())?))
        }
        else if self.equations.has_equation(t)
        {
            Ok(Some(self.modifiers.apply_modifiers(self, t, self.equations.eval(t, self)?)?))
        }
        else
        {
            Ok(None)
        }
    }

    /// Sets the value of an attribute directly. This should ONLY be
    /// used for initialization, as this circumvents the effect and modifier
    /// system.
    /// 
    /// This can fail if the given tag targets an existing value which is
    /// not an attribute or the attribute does not exist.
    /// 
    /// Returns the previous value if it existed
    pub fn set_attribute(&mut self, t: &Tag, nv: f32) -> Result<Option<f32>, DataError>
    {
        self.ensure_target_attribute(t)?;
        if let Some(a) = self.atrs.get_mut(t)
        {
            let old = a.get_value();
            a.set_value(nv);
            Ok(Some(old))
        }
        else
        {
            self.general_tags.add_tag(t);
            self.atrs.set_attribute(t, nv);
            Ok(None)
        }
    }

    /// Removes an attribute entirely from this dataset. Returns the value it used to be.
    /// 
    /// This should be used sparingly, as it completely removes an attribute
    /// from a dataset, so equations or conditionals that rely on this attribute
    /// value will no longer work and the dataset will error when attempting
    /// to evaluate those values.
    pub fn remove_attribute(&mut self, t: &Tag) -> Result<Option<f32>, DataError>
    {
        self.ensure_target_attribute(t)?;
        if self.atrs.has_attribute(t)
        {
            self.general_tags.remove_tag(t);
            Ok(self.atrs.remove_attribute(t).map(|a| a.get_value()))
        }
        else
        {
            Ok(None)
        }
    }

    /// If the given modifier is not already applied to this context,
    /// applies it. The old modifier is returned if the modifier was replaced
    pub fn set_modifier(&mut self, m: Modifier) -> Result<Option<Modifier>, DataError>
    {
        self.ensure_target_modifier(&m.name)?;
        let old = self.modifiers.get_modifier(&m.name).cloned();
        if old.is_some() 
        {
            self.modifiers.remove_modifier(&m.name);
        }
        else
        {
            self.general_tags.add_tag(&m.name);
        }
        self.modifiers.add_modifier(m);
        Ok(old)
    }

    pub fn remove_modifier(&mut self, t: &Tag) -> Result<Option<Modifier>, DataError>
    {
        self.ensure_target_modifier(&t)?;
        if self.has_modifier(t)
        {
            self.general_tags.remove_tag(t);
            Ok(self.modifiers.remove_modifier(t))
        }
        else
        {
            Ok(None)
        }
    }

    pub fn set_equation(&mut self, nv: Equation) -> Result<Option<Equation>, DataError>
    {
        self.ensure_target_equation(&nv.name)?;
        if let Some(e) = self.equations.get_mut(&nv.name)
        {
            let old = e.clone();
            *e = nv;
            Ok(Some(old))
        }
        else
        {
            self.general_tags.add_tag(&nv.name);
            self.equations.set_equation(nv);
            Ok(None)
        }
    }

    pub fn eval_equation(&self, equation_name: &Tag) -> Result<f32, DataError>
    {
        self.ensure_target_equation(equation_name)?;
        self.equations.eval(equation_name, self)
    }

    pub fn remove_equation(&mut self, equation_name: &Tag) -> Result<Option<Equation>, DataError>
    {
        self.ensure_target_equation(equation_name)?;
        if self.has_equation(equation_name)
        {
            self.general_tags.remove_tag(equation_name);
            Ok(self.equations.remove_equation(equation_name))
        }
        else
        {
            Ok(None)
        }
    }

    pub fn set_conditional(&mut self, nv: Conditional) -> Result<Option<Conditional>, DataError>
    {
        self.ensure_target_conditional(&nv.name)?;
        if let Some(c) = self.conditionals.get_mut(&nv.name)
        {
            let old = c.clone();
            *c = nv;
            Ok(Some(old))
        }
        else
        {
            self.general_tags.add_tag(&nv.name);
            self.conditionals.set_conditional(nv);
            Ok(None)
        }
    }

    pub fn eval_conditional(&self, conditional_name: &Tag) -> Result<bool, DataError>
    {
        self.ensure_target_conditional(conditional_name)?;
        self.conditionals.eval(conditional_name, self)
    }

    pub fn remove_conditional(&mut self, conditional_name: &Tag) -> Result<Option<Conditional>, DataError>
    {
        self.ensure_target_conditional(conditional_name)?;
        if self.has_conditional(conditional_name)
        {
            self.general_tags.remove_tag(conditional_name);
            Ok(self.conditionals.remove_conditional(conditional_name))
        }
        else
        {
            Ok(None)
        }
    }

    pub fn set_text_data(&mut self, text_name: &Tag, text_data: String) -> Result<Option<String>, DataError>
    {
        self.ensure_target_text(text_name)?;
        if !self.has_text(text_name)
        {
            self.general_tags.add_tag(text_name);
        }
        Ok(self.text_data.insert(text_name.clone(),text_data))
    }

    pub fn get_text_data(&self, text_name: &Tag) -> Result<Option<&String>, DataError>
    {
        self.ensure_target_text(text_name)?;
        Ok(self.text_data.get(text_name))
    }

    pub fn remove_text_date(&mut self, text_name: &Tag) -> Result<Option<String>, DataError>
    {
        self.ensure_target_text(text_name)?;
        Ok(self.text_data.remove(text_name))
    }

    /// Checks for attributes and equations which cause cycles
    /// of evaluation. For example:
    ///     attribute { name: test_atr }
    ///     conditional { name: test_cond, equation: test_atr == 3.0 }
    ///     modifier { name: test_mod, target: test_atr, conditional: test_cond }
    /// 
    /// If any cycles are found, the modifier's tag causing the cycle is returned.
    pub fn check_for_cyclic_evalutation(&self) -> Option<Vec<Tag>>
    {
        todo!()
    }

    fn ensure_target_attribute(&self, t: &Tag) -> Result<(), DataError>
    {
        self.ensure_target(t, DataType::Attribute)
    }

    fn ensure_target_modifier(&self, t: &Tag) -> Result<(), DataError>
    {
        self.ensure_target(t, DataType::Modifier)
    }

    fn ensure_target_equation(&self, t: &Tag) -> Result<(), DataError>
    {
        self.ensure_target(t, DataType::Equation)
    }

    fn ensure_target_conditional(&self, t: &Tag) -> Result<(), DataError>
    {
        self.ensure_target(t, DataType::Condition)
    }

    fn ensure_target_text(&self, t: &Tag) -> Result<(), DataError>
    {
        self.ensure_target(t, DataType::Text)
    }

    fn ensure_target(&self, t: &Tag, target: DataType) -> Result<(), DataError>
    {
        let conflict = if self.has_attribute(t)
        {
            Some(DataType::Attribute)
        }
        else if self.has_equation(t)
        {
            Some(DataType::Equation)
        }
        else if self.has_conditional(t)
        {
            Some(DataType::Condition)
        }
        else if self.has_modifier(t)
        {
            Some(DataType::Modifier)
        }
        else if self.has_text(t)
        {
            Some(DataType::Text)
        }
        else
        {
            None
        };

        if let Some(conflict) = conflict
        {
            if conflict == target
            {
                Ok(())
            }
            else
            {
                Err(DataError::ConflictingExpectedType(ConflictError::new(t.clone(), target, conflict)))
            }
        }
        else
        {
            Ok(())
        }
    }

    pub(crate) fn as_raw(&self) -> RawContextData
    {
        RawContextData
        {
            general_tags: self.general_tags.clone(),
            state_tags: self.state_tags.clone(),
            atrs: self.atrs.clone(),
            modifiers: self.modifiers.clone(),
            equations: self.equations.clone(),
            conditionals: self.conditionals.clone(),
            text_data: self.text_data.clone(),
        }
    }

    pub(crate) fn from_raw(raw: RawContextData) -> Result<Self, DataError>
    {
        let mut result = Self::new();
        result.state_tags = raw.state_tags;

        for (_, a) in raw.atrs
        {
            result.set_attribute(a.get_name(), a.get_value())?;
        }

        for (_, m) in raw.modifiers
        {
            result.set_modifier(m)?;
        }

        for (_, e) in raw.equations
        {
            result.set_equation(e)?;
        }

        for (_, c) in raw.conditionals
        {
            result.set_conditional(c)?;
        }

        for (t, text) in raw.text_data
        {
            result.set_text_data(&t, text)?;
        }

        Ok(result)
    }
}

impl From<&AttributeSet> for Context
{
    fn from(value: &AttributeSet) -> Self
    {
        let mut res = Context::new();
        for (t, a) in value.iter()
        {
            let _ = res.set_attribute(&t, a.get_value());
        }
        res
    }
}

/// Contains simply the raw data of a context. Useful for parsing and debug, should not be used for any major logic
pub struct RawContextData
{
    pub general_tags: TagSet,
    pub state_tags: TagSet,
    pub atrs: AttributeSet,
    pub modifiers: ModifierSet,
    pub equations: EquationSet,
    pub conditionals: ConditionalSet,
    pub text_data: HashMap<Tag, String>,
}