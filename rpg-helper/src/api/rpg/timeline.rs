use std::{cmp::Ordering, collections::HashSet, rc::Rc};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::api::{data::{attribute::AttributeSet, context::Context, equation::Equation, error::ParseError, tag::Tag}, rpg::event::{Event, EventInterval}};

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

    /// Produces a sorted list of events which are grouped by
    /// existing within the same interval identifier. The interval
    /// identifier is provided.
    pub fn split_by_interval(&self, interval: &EventInterval) -> Vec<(Tag, Vec<&Event>)>
    {
        todo!()
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

static LHS: Lazy<Result<Tag, ParseError>> = Lazy::new(|| Tag::from_str("lhs"));
static RHS: Lazy<Result<Tag, ParseError>> = Lazy::new(|| Tag::from_str("rhs"));

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct DateSpec
{
    pub ordering: Equation,
    pub required_values: HashSet<Tag>,
}

impl DateSpec
{
    pub fn new(ordering: Equation, required_values: HashSet<Tag>) -> DateSpec
    {
        DateSpec { ordering, required_values }
    }

    pub fn get_ordering_lhs_tag() -> &'static Tag
    {
        match &*LHS
        {
            Ok(lhs) => lhs,
            Err(e) => panic!("LHS tag of Date Spec is not valid!\nFailed with error: {:?}", e),
        }
    }

    pub fn get_ordering_rhs_tag() -> &'static Tag
    {
        match &*RHS
        {
            Ok(rhs) => rhs,
            Err(e) => panic!("RHS tag of Date Spec is not valid!\nFailed with error: {:?}", e),
        }
    }
}

impl Default for DateSpec
{
    /// Not my favorite thing to do right now, but we need a default for testing
    fn default() -> Self
    {
        let mut required_values = HashSet::new();
        required_values.insert(Tag::from_str("Year").unwrap());
        required_values.insert(Tag::from_str("Month").unwrap());
        required_values.insert(Tag::from_str("Day").unwrap());
        Self { ordering: Equation::new(Tag::from_str("ordering").unwrap(), "(rhs.Year - lhs.Year) * 365 + (rhs.Month - lhs.Month) * 30 + (rhs.Day - lhs.Day)").unwrap(), required_values }
    }
}

/// It might be good to define the date spec as
/// tag, a reference to the ordering equation.
/// This way, a date could be a simple Copy value
/// This works for now though.
#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Date
{
    ordering: Equation,
    values: AttributeSet,
}

impl PartialOrd for Date
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
    {
        const EPSILON: f32 = 0.0000001;



        let (lhs_prefix, rhs_prefix) = match (&*LHS, &*RHS)
        {
            (Ok(lhs), Ok(rhs)) => (lhs, rhs),
            _ => return None,
        };

        // Doing some cloning, but attribute sets on dates are typically very small so doesn't really matter
        let mut ctx: Context = self.values.clone().add_prefix(lhs_prefix).into();
        
        if let Err(_) = ctx.layer_context(&other.values.clone().add_prefix(rhs_prefix).into())
        {
            return None;
        }

        if let Ok(comparison_value) = self.ordering.eval(&ctx)
        {
            if comparison_value < EPSILON
            {
                Some(std::cmp::Ordering::Less)
            }
            else if comparison_value > EPSILON
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
}