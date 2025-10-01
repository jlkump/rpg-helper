use std::{cmp::Ordering, collections::{HashMap, HashSet}, rc::Rc};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::api::{data::{attribute::AttributeSet, context::Context, equation::Equation, error::ParseError, tag::{Subtag, Tag}}, rpg::event::{Event}};

/// A simple wrapper around an array of events
/// When owned by a character, the timeline represents
/// the local time experience of the character. Events
/// do not have to be in chronoligical order by the date
/// ordering (although the front-end client will facilitate
/// the creation of events such that this is handled to avoid
/// chronological errors).
#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Timeline
{
    events: Vec<Event>,
}

impl Timeline
{
    pub fn add_event(&mut self, e: Event)
    {
        self.events.push(e);
    }

    pub fn insert_event(&mut self, index: usize, e: Event)
    {
        self.events.insert(index, e);
    }

    pub fn iter(&self) -> impl Iterator<Item = &Event>
    {
        self.events.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Event>
    {
        self.events.iter_mut()
    }

    pub fn group_by_time_context(&self) -> Vec<(Tag, Vec<&Event>)>
    {
        todo!()
    }
}

/// An identifier for determining what timeline a character exists on.
/// This is used for determining event intervals and resource conflicts,
/// as well as resource sharing. For example, a character can only share
/// resources while in the same time context as another character.
/// 
/// The time context is simply a tag identifier.
/// Examples:
///     timeline.mundane
///     timeline.fay
///     timeline.heaven
///     timeline.hell
#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum TimeContext
{
    Calendar,
    Temporary,
}

/// Occurances (yearly, weekly, daily special events) are tracked on a calendar
/// 
/// A calendar is a collection of Days, which are grouped up as seen fit by the
/// user creating a ruleset. The important thing is that days are given a u16 index
/// in the array of Calendar days.
/// 
/// This allows an Event Interval for a Calendar context to be defined by an
/// interval of days in a year [start, end).
pub struct Calendar
{
    time_ctx_id: Subtag,
    days: Vec<Day>,
    // This ctx is appended to the ruleset context with the prefix
    // timeline.[time_ctx_id].*
    // The current day and year are stored in:
    // timeline.[time_ctx_id].year
    // timeline.[time_ctx_id].day
    // 
    // The calendar can contain special events
    // under timeline.[time_ctx_id].events
    // which are a list of conditionals depending
    // on the day and year value.
    //
    // These conditionals could be used for astronomical events, such as
    // whether the moon is full, waning, waxing, etc. These can be read by
    // locations and characters to be used for special abilities or resources
    // that depend upon timing.
    // 
    // These conditionals can also be used to mark future story events? Maybe
    ctx: Context,
    intervals: Vec<EventInterval>,
}

pub struct Day
{
    // A day could be conditional, such as the leap year day. Such a condition
    // uses timeline.[time_ctx_id].year as the only accessible value
    
    name: Option<Subtag>,
}

/// The event interval is the range of time over which
/// resources are limited. 
/// 
/// For example, ars magica lets players share books, but
/// not during the same season. Thus, each season
/// would be defined as an event interval and books as a
/// resource would have a share limit of 1.
/// 
/// For this to work, an EventInterval compares two dates
/// to see if the two dates are considered in the same event
/// interval.
pub struct EventInterval
{
    start: u16,
    end: u16,
}

/// Dates are always measured in the context of a game, within a Time Context.
/// The year value represents the time before or after the start date of the game.
#[derive(Debug, Deserialize, PartialEq, Eq, Ord, Serialize, Clone, Copy)]
pub struct Date
{
    time_ctx_id: Subtag,
    // The offset to the start year in the ruleset for the active game.
    year: i16,
    day: u16,
}

/// Ordering assumes that dates have a matching time context
/// If this is not the case, then the partial order will return None
impl PartialOrd for Date
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        // These dates exist in separate timeline contexts
        // Comparing them is meaningless
        if self.time_ctx_id != other.time_ctx_id { return None; }

        // Order by year, then by day
        match self.year.partial_cmp(&other.year)
        {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.day.partial_cmp(&other.day)
    }
}