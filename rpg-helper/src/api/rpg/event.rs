use std::{collections::HashMap, rc::Rc};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::api::{data::{conditional::Conditional, context::Context, effect::Effect, error::{DataError, ParseError}, tag::{Subtag, Tag, TagTemplate}}, rpg::{ability::Ability, inventory::Item, timeline::Date}, };

/// This is an instance of an Event using specifications from the EventSchema.
/// It holds the date it took place and all the modifications performed.
/// NOTE: If event schemas are changed, the associated Event will NOT be changed.
#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Event
{
    pub schema: Tag,            // Reference to the type that made this event
    pub id: Tag,                // The identifier of this event in particular
    pub date: Date,
    pub ctx: Context,           // This is the additional ctx which was active during
                                // the creation of this event. It should be fairly small, as it
                                // represents values such as the calculation of event values
    modifications: Vec<EventModification>,
}

impl Event
{
    pub fn get_event_modifications(&self, character_ctx: &Context) -> Vec<EventModification>
    {
        todo!()
    }
}

impl PartialOrd for Event
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
    {
        self.date.partial_cmp(&other.date)
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum EventModification
{
    AddProgress(Tag, Tag, Option<(f32, f32)>),   // First tag is target to add to, second tag is the value to add to, optional clamped values
    CheckProgress(Tag, Vec<EventModification>),  // First tag is the conditional to check, second is the vec of modifications to apply if the check succeeds.
    ClearProgress(Tag),                         // Clears the progress value. Useful on the completion of progress
    AddToAttribute(Tag, f32),
    GrantAbility(Ability),                              // Grant an ability to the player. The ability is created in the process of creating this event, which is why it isn't defined from values within the event ctx
    GiveItem(Item),                                  // Gives an item. Like ability, the item is defined in the creation of the event
    RevokeAbility(Tag),                          // Removes an ability by the id tag of the individual ability.
    RemoveItem(Tag),
    // This is an event that only really matters for the character individually, so it will not typically be displayed on a global timeline.
    ChangeTimeContext(Subtag),
}

/// An event schema is used to create an event during active gameplay.
/// It contains the specifications for how to make an event of
/// a specific type.
/// 
/// For example, ars magica lets players teach abilities and arts to each other.
/// For this, a player's character would be marked as a resource, with the resource's
/// tags being prefixed with "teacher". We would then querry for the abilities and arts
/// of the teacher with the prefix "teacher.attribute.ability" and "teacher.attribute.art".
/// 
/// The player would be able to choose a value as long as certain restrictions are met.
/// Namely, having a shared language and the teacher having a higher score than the player.
/// 
/// NOTE: While restrictions to tag types are defined here, it is up to
/// the implmentation of the client and server to ensure that the restrictions
/// are upheld. An event schema can be turned into an event without restrictions
/// enforced (in this layer of the API)
pub struct EventSchema
{
    pub id: Tag,
    
    // template_tags: Vec<TagTemplate>,
    // template_attributes: Vec<TagTemplate>,
    // template_equations: Vec<TagTemplate>,
    // template_conditions: Vec<TagTemplate>,
}

impl EventSchema
{

}


/// A resource is some set of values (in a ctx)
/// that is available as a choice during the creation of
/// events.
/// 
/// They represent some shared values that are provided
/// by a location, items, or other characters.
/// 
/// All resources are prefixed with
/// "resource.resource name" when
/// layered with another ctx.
pub struct Resource
{
    ctx: Context,
}

impl Resource
{
    /// Return a value representing
    /// the maximum number of times this resource can
    /// be used in a single event interval
    pub fn get_share_limit(&self) -> Result<i32, DataError>
    {
        static SHARE_LIMIT_TAG: Lazy<Tag> = Lazy::new(|| Tag::from_str("share limit").unwrap());
        if let Ok(Some(v)) = self.ctx.get_value(&SHARE_LIMIT_TAG)
        {
            Ok(v as i32)
        }
        else
        {
            Err(DataError::InvalidState("Resource does not contain \'share limit\' attribute".to_string()))
        }
    }
}