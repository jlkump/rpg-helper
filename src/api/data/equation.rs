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

impl Equation
{
    pub fn new(name: Tag, equation: &str) -> Result<Equation, DataError>
    {
        Ok(Equation { name, equation_string: equation.to_string(), ast: EvalTree::from_str(equation)? })
    }
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

    pub fn get(&self, equation_name: &Tag) -> Option<&Equation>
    {
        self.equations.get(equation_name)
    }

    pub fn get_mut(&mut self, equation_name: &Tag) -> Option<&mut Equation>
    {
        self.equations.get_mut(equation_name)
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

    pub fn has_equation(&self, equation_name: &Tag) -> bool
    {
        self.equations.contains_key(equation_name)
    }

    pub fn set_equation(&mut self, equation: Equation) -> Option<Equation>
    {
        self.equations.insert(equation.name.clone(), equation)
    }

    pub fn remove_equation(&mut self, equation_name: &Tag) -> Option<Equation>
    {
        self.equations.remove(equation_name)
    }
}