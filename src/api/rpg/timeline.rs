use std::{collections::HashSet, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::api::data::{attribute::AttributeSet, equation::Equation, tag::Tag};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Timeline
{

}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct DateSpec
{
    ordering: Equation,
    required_values: HashSet<Tag>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Date
{
    spec: Rc<DateSpec>,
    values: AttributeSet,
}

impl PartialOrd for Date
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
    {
        let ctx = &self.values;
        if let Ok(date_val) = self.spec.ordering.eval(&ctx.into())
        { 
            let ctx = &other.values;
            if let Ok(other_date_val) = other.spec.ordering.eval(&ctx.into())
            {
                if date_val < other_date_val
                {
                    Some(std::cmp::Ordering::Less)
                }
                else if date_val > other_date_val
                {
                    Some(std::cmp::Ordering::Greater)
                }
                else
                {
                    Some(std::cmp::Ordering::Equal)
                }
            }
            else
            {
                None
            }
        }
        else
        {
            None
        }
    }
}