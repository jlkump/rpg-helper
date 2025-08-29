use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::api::data::{context::Context, error::{DataError, DoesNotExistError, TemplateError}, evaltree::EvalTree, tag::{Tag, TagTemplate}, template::{Template, Templated}};

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
        let ast = EvalTree::from_str(equation)?;
        if ast.is_template()
        {
            return Err(DataError::StringInputInvalid(format!("Given equation \"{}\" contains template values. An equation can not contain template values.", equation)));
        }
        Ok(Equation { name, equation_string: equation.to_string(), ast })
    }

    pub fn eval(&self, ctx: &Context) -> Result<f32, DataError>
    {
        self.ast.eval_as_num(ctx)
    }

    pub fn get_equation_string(&self) -> String
    {
        self.equation_string.clone()
    }

    pub fn check_only_allowed_tags(&self, allowed_tags: &Vec<Tag>) -> Result<(), Templated<TagTemplate, Tag>>
    {
        self.ast.check_only_allowed_tags(allowed_tags)
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

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct EquationTemplate
{
    name_template: Templated<TagTemplate, Tag>,
    /// This templated string will NOT change
    /// while input values are placed in a equation template.
    /// Instead, the final string in creation of the equation is created
    /// from the filled in ast.
    templated_equation_string: String,
    ast: EvalTree,
}

impl EquationTemplate
{
    pub fn new(name: &str, equation: &str) -> Result<Templated<EquationTemplate, Equation>, DataError>
    {
        let name_template = TagTemplate::from_str(name)?;
        let name_template = if name_template.get_required_inputs().is_empty()
        {
            Templated::Complete(name_template.into_tag()?)
        }
        else
        {
            Templated::Template(name_template)
        };

        let ast = EvalTree::from_str(equation)?;
        if !name_template.is_complete() || ast.is_template()
        {
            return Ok(Templated::Template(EquationTemplate { name_template, templated_equation_string: equation.to_string(), ast }));
        }

        if let Some(name) = name_template.into_complete()
        {
            Ok(Templated::Complete(Equation { name, equation_string: ast.to_expression_string(), ast }))
        }
        else
        {
            Err(DataError::InvalidState(format!("Unreachable state in equation template creation")))
        }

    }
}

impl Template<Equation> for EquationTemplate
{
    fn get_required_inputs(&self) -> std::collections::HashSet<String>
    {
        let mut result = self.name_template.get_required_inputs();
        result.extend(self.ast.get_template_inputs());
        result
    }

    fn fill_template_value(&mut self, input_name: &str, input_value: &Tag) -> Option<Equation>
    {
        self.name_template.fill_template_value(input_name, input_value);
        self.ast.insert_template_input(input_name, input_value);
        

        if !self.ast.is_template()
        {
            if let Some(name) = self.name_template.as_complete()
            {
                return Some(Equation { name: name.clone(), equation_string: self.ast.to_expression_string(), ast: self.ast.clone() });
            }
        }
        None
    }

    fn attempt_complete(&self) -> Result<Equation, super::error::TemplateError>
    {
        if self.ast.is_template()
        {
            let mut result = self.ast.get_template_inputs();
            result.extend(self.name_template.get_required_inputs());
            return Err(TemplateError::MissingTemplateValues(result));    
        }

        let name = match &self.name_template
        {
            Templated::Template(t) => t.attempt_complete()?,
            Templated::Complete(c) => c.clone(),
        };

        Ok(Equation { name, equation_string: self.ast.to_expression_string(), ast: self.ast.clone() })
    }
}