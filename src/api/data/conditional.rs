use std::collections::HashMap;

use crate::api::data::{error::{DataError, DoesNotExistError}, evaltree::EvalTree, tag::Tag, Context};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Conditional
{
    name: Tag,
    equation_string: String,
    ast: EvalTree,
}

impl Conditional
{
    pub fn new(name: Tag, equation: &str) -> Result<Conditional, DataError>
    {
        Ok(Conditional { name, equation_string: equation.to_string(), ast: EvalTree::from_str(equation)? })
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct ConditionalSet
{
    conditionals: HashMap<Tag, Conditional>,
}

impl ConditionalSet
{
    pub fn new() -> ConditionalSet
    {
        ConditionalSet { conditionals: HashMap::new() }
    }

    pub fn get(&self, conditional_name: &Tag) -> Option<&Conditional>
    {
        self.conditionals.get(conditional_name)
    }

    pub fn get_mut(&mut self, conditional_name: &Tag) -> Option<&mut Conditional>
    {
        self.conditionals.get_mut(conditional_name)
    }

    pub fn eval(&self, conditional_name: &Tag, ctx: &Context) -> Result<bool, DataError>
    {
        if let Some(c) = self.conditionals.get(conditional_name)
        {
            c.ast.eval_as_bool(ctx)
        }
        else
        {
            Err(DataError::DoesNotExist(DoesNotExistError::Condition(conditional_name.clone())))
        }
    }

    pub fn has_conditional(&self, conditional_name: &Tag) -> bool
    {
        self.conditionals.contains_key(conditional_name)
    }

    pub fn set_conditional(&mut self, conditional: Conditional) -> Option<Conditional>
    {
        self.conditionals.insert(conditional.name.clone(), conditional)
    }

    pub fn remove_conditional(&mut self, conditional_name: &Tag) -> Option<Conditional>
    {
        self.conditionals.remove(conditional_name)
    }
}