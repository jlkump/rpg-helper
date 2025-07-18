use std::{cmp::Ordering, collections::HashSet, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::api::{data::{attribute::AttributeSet, equation::Equation, tag::Tag}, rpg::event::Event};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Timeline
{
    events: Vec<Event>,  // Events are kept in sorted order
}

impl Timeline
{
    pub fn add_event(&mut self, e: Event)
    {
        self.events.push(e);
        self.events.sort_by(|l, r| l.partial_cmp(r).unwrap_or(Ordering::Equal));
    }

    pub fn iter(&self) -> impl Iterator<Item = &Event>
    {
        self.events.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Event>
    {
        self.events.iter_mut()
    }

    /// Add the events of both timelines together
    /// Returns the resultant combination of events in a new timeline
    pub fn combine(&self, other: &Self) -> Self
    {
        let mut result_events = self.events.clone();
        for o in other.events.iter()
        {
            result_events.push(o.clone());
        }
        result_events.sort_by(|l, r| l.partial_cmp(r).unwrap_or(Ordering::Equal));
        Timeline { events: result_events }
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct DateSpec
{
    ordering: Equation,
    required_values: HashSet<Tag>,
}

/// It might be good to define the date spec as
/// tag, a reference to the ordering equation.
/// This way, a date could be a simple Copy value
/// This works for now though.
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
        let lhs_prefix = match Tag::from_str("lhs")
        {
            Ok(tag) => tag,
            Err(_) => return None,
        };

        let rhs_prefix = match Tag::from_str("rhs")
        {
            Ok(tag) => tag,
            Err(_) => return None,
        };

        // Doing some cloning, but attribute sets on dates are typically very small so doesn't really matter
        let ctx = self.values.clone().add_prefix(&lhs_prefix);
        if let Ok(date_val) = self.spec.ordering.eval(&(&ctx).into())
        { 
            let ctx = other.values.clone().add_prefix(&rhs_prefix);
            if let Ok(other_date_val) = other.spec.ordering.eval(&(&ctx).into())
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