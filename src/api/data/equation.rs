use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::api::data::{error::{DataError, DoesNotExistError}, evaltree::EvalTree, tag::Tag, Context};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Equation
{
    name: Tag,
    equation_string: String,
    ast: EvalTree,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct EquationSet
{
    equations: HashMap<Tag, Equation>,
}

impl EquationSet
{
    pub fn new() -> EquationSet
    {
        EquationSet { equations: HashMap::new() }
    }

    pub fn eval(&self, equation_name: &Tag, ctx: &Context) -> Result<f32, DataError>
    {
        if let Some(e) = self.equations.get(equation_name)
        {
            e.ast.eval_as_num(ctx)
        }
        else
        {
            Err(DataError::DoesNotExist(DoesNotExistError::Equation(equation_name.clone())))
        }
    }

    pub fn can_eval(&self, equation_name: &Tag) -> bool
    {
        self.equations.contains_key(equation_name)
    }
}