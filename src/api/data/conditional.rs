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
}