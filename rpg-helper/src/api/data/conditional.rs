use std::collections::HashMap;

use crate::api::data::{context::Context, error::{DataError, DoesNotExistError, TemplateError}, evaltree::EvalTree, tag::{Tag, TagTemplate}, template::{Template, Templated}};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Conditional
{
    pub name: Tag,
    equation_string: String,
    ast: EvalTree,
}

impl Conditional
{
    pub fn new(name: Tag, equation: &str) -> Result<Conditional, DataError>
    {
        let ast = EvalTree::from_str(equation)?;
        if ast.is_template()
        {
            return Err(DataError::StringInputInvalid(format!("Given equation \"{}\" contains template values. A conditional can not contain template values.", equation)));
        }
        Ok(Conditional { name, equation_string: equation.to_string(), ast })
    }

    pub fn eval(&self, ctx: &Context) -> Result<bool, DataError>
    {
        self.ast.eval_as_bool(ctx)
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
            c.eval(ctx)
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

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, Tag, Conditional>
    {
        self.conditionals.iter()
    }
}

impl IntoIterator for ConditionalSet
{
    type Item = (Tag, Conditional);

    type IntoIter = std::collections::hash_map::IntoIter<Tag, Conditional>;

    fn into_iter(self) -> Self::IntoIter
    {
        self.conditionals.into_iter()
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct ConditionalTemplate
{
    name_template: Templated<TagTemplate, Tag>,
    /// This templated string will NOT change
    /// while input values are placed in a conditional template.
    /// Instead, the final string in creation of the conditional is created
    /// from the filled in ast.
    templated_equation_string: String,
    ast: EvalTree,
}

impl ConditionalTemplate
{
    pub fn new(name: &str, equation: &str) -> Result<Templated<ConditionalTemplate, Conditional>, DataError>
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
            return Ok(Templated::Template(ConditionalTemplate { name_template, templated_equation_string: equation.to_string(), ast }));
        }

        if let Some(name) = name_template.into_complete()
        {
            Ok(Templated::Complete(Conditional { name, equation_string: ast.to_expression_string(), ast }))
        }
        else
        {
            Err(DataError::InvalidState(format!("Unreachable state in conditional template creation")))
        }

    }
}

impl Template<Conditional> for ConditionalTemplate
{
    fn get_required_inputs(&self) -> std::collections::HashSet<String>
    {
        let mut result = self.name_template.get_required_inputs();
        result.extend(self.ast.get_template_inputs());
        result
    }

    fn fill_template_value(&mut self, input_name: &str, input_value: &Tag) -> Option<Conditional>
    {
        self.name_template.fill_template_value(input_name, input_value);
        self.ast.insert_template_input(input_name, input_value);
        

        if !self.ast.is_template()
        {
            if let Some(name) = self.name_template.as_complete()
            {
                return Some(Conditional { name: name.clone(), equation_string: self.ast.to_expression_string(), ast: self.ast.clone() });
            }
        }
        None
    }

    fn attempt_complete(&self) -> Result<Conditional, super::error::TemplateError>
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

        Ok(Conditional { name, equation_string: self.ast.to_expression_string(), ast: self.ast.clone() })
    }
}