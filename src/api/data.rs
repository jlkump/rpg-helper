use crate::api::data::{attribute::AttributeSet, conditional::{Conditional, ConditionalSet}, effect::Effect, equation::{Equation, EquationSet}, error::{ConflictError, DataError, DataType}, modifier::{Modifier, ModifierSet}, tag::{Tag, TagSet}};

use serde::{Deserialize, Serialize};

pub mod attribute;
pub mod conditional;
pub mod effect;
pub mod error;
pub mod evaltree;
pub mod equation;
pub mod modifier;
pub mod tag;

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
    tags: TagSet,
    atrs: AttributeSet,
    modifiers: ModifierSet,
    equations: EquationSet,
    conditionals: ConditionalSet,
}

impl Context
{
    pub fn new() -> Context
    {
        Context { 
            tags: TagSet::new(), 
            atrs: AttributeSet::new(), 
            modifiers: ModifierSet::new(),
            equations: EquationSet::new(),
            conditionals: ConditionalSet::new(), 
        }
    }

    pub fn has_tag(&self, t: &Tag) -> bool
    {
        self.tags.has_tag(t)
    }

    pub fn has_attribute(&self, attribute_name: &Tag) -> bool
    {
        self.atrs.has_attribute(attribute_name)
    }

    pub fn has_conditional(&self, conditional_name: &Tag) -> bool
    {
        self.conditionals.has_conditional(conditional_name)
    }

    pub fn has_modifier(&self, modifier_name: &Tag) -> bool
    {
        self.modifiers.has_modifier(modifier_name)
    }

    pub fn has_equation(&self, attribute_alias: &Tag) -> bool
    {
        self.equations.can_eval(attribute_alias)
    }

    /// Creates a new context that combines this context plus another context.
    /// If there are conflicting keys, the values of "other" are perfered
    /// over self
    pub fn layer_context(&self, other: &Self) -> Self
    {
        todo!()
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
        else if self.equations.can_eval(t)
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
            self.tags.add_tag(t);
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
            self.tags.remove_tag(t);
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
            self.tags.add_tag(&m.name);
        }
        self.modifiers.add_modifier(m);
        Ok(old)
    }

    pub fn remove_modifier(&mut self, t: &Tag) -> Result<Option<Modifier>, DataError>
    {
        self.ensure_target_modifier(&t)?;
        if self.has_modifier(t)
        {
            self.tags.remove_tag(t);
            Ok(self.modifiers.remove_modifier(t))
        }
        else
        {
            Ok(None)
        }
    }

    pub fn set_equation(&self, t: &Tag, e: Equation) -> Result<Option<Equation>, DataError>
    {
        todo!()
    }

    pub fn eval_equation(&self, t: &Tag) -> Result<f32, DataError>
    {
        self.ensure_target_equation(t)?;
        self.equations.eval(t, self)
    }

    pub fn remove_equation(&mut self, t: &Tag) -> Result<Option<Equation>, DataError>
    {
        todo!()
    }

    pub fn set_conditional(&self, t: &Tag, c: Conditional) -> Result<Option<Conditional>, DataError>
    {
        todo!()
    }

    pub fn eval_conditional(&self, t: &Tag) -> Result<bool, DataError>
    {
        self.ensure_target_conditional(t)?;
        if self.has_conditional(t)
        {
            Ok(self.conditionals.eval(t, self))
        }
        else
        {
            Err(DataError::condition_dne(t.clone()))
        }
    }

    pub fn remove_conditional(&mut self, t: &Tag) -> Result<Option<Conditional>, DataError>
    {
        todo!()
    }

    fn ensure_target_attribute(&self, t: &Tag) -> Result<(), DataError>
    {
        if self.has_conditional(t)
        {
            return Err(DataError::ConflictingExpectedType(ConflictError::new(t.clone(), DataType::Attribute, DataType::Condition)));
        }
        else if self.has_modifier(t)
        {
            return Err(DataError::ConflictingExpectedType(ConflictError::new(t.clone(), DataType::Attribute, DataType::Modifier)));
        }
        else if self.has_equation(t)
        {
            return Err(DataError::ConflictingExpectedType(ConflictError::new(t.clone(), DataType::Attribute, DataType::Equation)));
        }
        else
        {
            Ok(())
        }
    }

    fn ensure_target_modifier(&self, t: &Tag) -> Result<(), DataError>
    {
        if self.has_conditional(t)
        {
            return Err(DataError::ConflictingExpectedType(ConflictError::new(t.clone(), DataType::Attribute, DataType::Condition)));
        }
        else if self.has_attribute(t)
        {
            return Err(DataError::ConflictingExpectedType(ConflictError::new(t.clone(), DataType::Attribute, DataType::Attribute)));
        }
        else if self.has_equation(t)
        {
            return Err(DataError::ConflictingExpectedType(ConflictError::new(t.clone(), DataType::Attribute, DataType::Equation)));
        }
        else
        {
            Ok(())
        }
    }

    fn ensure_target_equation(&self, t: &Tag) -> Result<(), DataError>
    {
        if self.has_modifier(t)
        {
            return Err(DataError::ConflictingExpectedType(ConflictError::new(t.clone(), DataType::Attribute, DataType::Modifier)));
        }
        else if self.has_attribute(t)
        {
            return Err(DataError::ConflictingExpectedType(ConflictError::new(t.clone(), DataType::Attribute, DataType::Attribute)));
        }
        else if self.has_conditional(t)
        {
            return Err(DataError::ConflictingExpectedType(ConflictError::new(t.clone(), DataType::Attribute, DataType::Condition)));
        }
        else
        {
            Ok(())
        }
    }

    fn ensure_target_conditional(&self, t: &Tag) -> Result<(), DataError>
    {
        if self.has_modifier(t)
        {
            return Err(DataError::ConflictingExpectedType(ConflictError::new(t.clone(), DataType::Attribute, DataType::Modifier)));
        }
        else if self.has_attribute(t)
        {
            return Err(DataError::ConflictingExpectedType(ConflictError::new(t.clone(), DataType::Attribute, DataType::Attribute)));
        }
        else if self.has_equation(t)
        {
            return Err(DataError::ConflictingExpectedType(ConflictError::new(t.clone(), DataType::Attribute, DataType::Equation)));
        }
        else
        {
            Ok(())
        }
    }
}