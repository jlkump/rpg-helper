use serde::{Deserialize, Serialize};

use crate::api::{data::tag::Tag, rpg::dice::{DiceRollSpec, DieRollResult}};

/// Here we define the specifications for player input
/// This comes in use for abilities, events, and character creation
/// It is used for in-game character modification.
#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum PlayerInputAction
{
    ChooseTag(Vec<Tag>),            // Choose a tag from the given options
    PerformRoll(DiceRollSpec),      // Perform a type of roll
    SetValue(Option<(f32, f32)>),   // Optional clamp of value
    SetBool,                        
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum PlayerInputActionResponse
{
    ChooseTag(Tag),                    // The chosen tag
    PerformRoll(Vec<DieRollResult>),   // The roll result
    SetValue(f32),                     // The chosen value
    SetBool(bool),                     // The chosen value
}