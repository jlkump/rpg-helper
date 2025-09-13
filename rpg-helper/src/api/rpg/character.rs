// A character contains a dataset
// which acts as the collection of attributes and state-based tags
// representing the character.
//
// The character's dataset derives from the ruleset's
// base dataset (the character's dataset layers ontop the rulset's)
//
// Ars Magica example:
//    The ruleset defines the equation template for Abilities and Arts
//    Defines character templates, which are the base-dataset from which
//    characters derive their attributes, equations, and conditionals.


// Equation templates 
//          Defines a string with values that 
//          are replaced with tag values based on input.
//   Ex:
//      template = "rounddown((sqrt(8 * [EXP] / 5 + 1)-1)/2)"
//      fill_template(template, params: HashMap<String, Tag>) -> String,

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::api::{data::{context::Context, effect::Effect, error::DataError, tag::Tag}, rpg::{ability::{Ability, AbilitySet}, event::Event, timeline::{Date, Timeline}}};

// First todo:
//      1. Parse json in order to import character data
//      Example:
//
//      {
//          "ruleset": {
//              "id": "",
//              "version": "1.0.0"
//          },
//          "data":
//          {
//              "state_tags": ["tag", "tag.two"],
//              "attributes": [{"name": "attribute.name", "value": "0.0"}],
//              "modifiers": [{"name": "modifier.name", "target": "attribute.name", "condition": "condition.name", "change": "3.0"}],
//              "equations": [{"name": "equation.name", "equation": "attribute.name + 3.0"}],
//              "conditionals": [{"name": "conditional.name", "conditional": "equation.name == 3.0"}],
//              "text_data": [{"name", "Test Character"}, {"name.alias.0": "First Character"}],
//          },
//          "timeline": [{"event_name": "event_temp_name", "date": ...}],
//          "inventory": [{"item_name": "cool item", "item_tag": "cool_item_tag", "item_spec": 
//                         "spec_tag", "item_count": "1", "item_context": {...}}],
//          "equiped_items": ["cool_item_tag"],
//      }    

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Character
{
    data: CharacterData,    // The character's base starting data
    timeline: Timeline,     // All the changes applied to character-creation data
    current_date: Date,
    context_data: Context,  // Additional context data applied not through the timeline (ruleset data)

    // Whenever we change the current date, the final data of the character changes
    // This is the data we actually read for the purposes of gameplay.
    // The cached data is set to None whenever the
    // cache is invalidated.
    cached_final_data: Option<CharacterData>,
}

/// The character state tracks the less
/// impactful changes to the character that modify
/// the character's ctx, but not enough to be tracked
/// on the timeline.
/// 
/// For example, what items are actively equiped in a slot
/// and what abilities are active.
/// 
/// This is tracked in the timeline when an event is created,
/// thus, when the player goes to dates in the timeline,
/// the character state is adjusted accordingly.
struct CharacterState
{
    equiped_items: HashMap<Tag, Tag>,    // Map from slot to item.
    active_abilities: Vec<Tag>,          // Abilities active
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
struct CharacterData
{
    ctx: Context,
    abilities: AbilitySet,
    // inventory: Inventory,
}

impl Character
{
    /// Sets the active current date for the character.
    /// This changes the data of the character
    /// according to the events applied to the character.
    pub fn set_date(&mut self, date: Date)
    {
        self.current_date = date;
        self.cached_final_data = None;
    }

    pub fn get_date(&self) -> &Date
    {
        &self.current_date
    }

    /// Changes the data of a character. For the event to take place,
    /// the event's date must occur before the set current date.
    pub fn add_event(&mut self, event: Event)
    {
        self.timeline.add_event(event);
        self.cached_final_data = None;
    }
    
    pub fn get_timeline(&self) -> &Timeline
    {
        &self.timeline
    }

    /// Used to layer additional data, such as equations
    /// from a ruleset
    pub fn layer_ctx(mut self, ctx: &Context) -> Result<Self, DataError>
    {
        self.context_data.layer_context(&ctx)?;
        self.update_final_data()?;
        Ok(self)
    }

    /// Given a prefix tag, gets all immediate sub-tag values with that prefix
    /// For example, given the prefix "value.ability",
    /// retrives the value "value.ability.Magic Theory" but not "value.ability.Magic Theory.Exp"
    /// This is useful for display when we know we want to display all values of a given prefix type
    /// such as abilities or characteristics.
    pub fn get_values_of_prefix(&self, prefix: &Tag) -> Result<Vec<(Tag, f32)>, DataError>
    {
        todo!()
    }

    fn update_final_data(&mut self) -> Result<(), DataError>
    {
        // Change the character's data based on the current year and all timeline data
        let mut final_data = self.data.clone();
        final_data.ctx.layer_context(&self.context_data)?;

        // We create an empty timeline context which will be used
        // as a "scratch pad" of sorts for events.
        // This is useful for values such as the progress of completion for
        // a crafting of an item.

        for e in self.timeline.iter()
        {
            if e.date <= self.current_date
            {
                final_data.apply_event(e)?;
            }
            else
            {
                // We can end early, as we know the rest of the list will only be greater
                break;
            }
        }

        // Save resultant cached_character
        self.cached_final_data = Some(final_data);
        Ok(())
    }
}

impl CharacterData
{
    /// Actually apply the changes of an event to the data of this character.
    fn apply_event(&mut self, event: &Event) -> Result<(), DataError>
    {
        // for eff in event.get_character_mods(&self.ctx).iter()
        // {
        //     match &eff
        //     {
        //         CharacterModification::Effect(effect) =>
        //         {
        //             self.ctx.apply_effect(&effect.effect)?;
        //         },
        //         CharacterModification::Ability(ability_modification) =>
        //         {
        //             todo!()
        //         },
        //         CharacterModification::Item(item_modification) =>
        //         {
        //             todo!()
        //         },
        //     }
        // }
        Ok(())
    }
}