use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::api::data::{error::{DataError, DoesNotExistError}, evaltree::EvalTree, tag::Tag, context::Context};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Equation
{
    pub name: Tag,
    equation_string: String,
    ast: EvalTree,
}

impl Equation
{
    pub fn new(name: Tag, equation: &str) -> Result<Equation, DataError>
    {
        Ok(Equation { name, equation_string: equation.to_string(), ast: EvalTree::from_str(equation)? })
    }

    pub fn eval(&self, ctx: &Context) -> Result<f32, DataError>
    {
        self.ast.eval_as_num(ctx)
    }

    pub fn get_equation_string(&self) -> String
    {
        self.equation_string.clone()
    }

    pub fn is_template(&self) -> bool
    {
        todo!()
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
            e.eval(ctx)
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

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, Tag, Equation>
    {
        self.equations.iter()
    }
}

impl IntoIterator for EquationSet
{
    type Item = (Tag, Equation);

    type IntoIter = std::collections::hash_map::IntoIter<Tag, Equation>;

    fn into_iter(self) -> Self::IntoIter
    {
        self.equations.into_iter()
    }
}