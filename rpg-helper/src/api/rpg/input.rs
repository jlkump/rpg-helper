use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::api::{data::tag::Tag, rpg::dice::{DiceRoll, DiceRollResult}};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum InputAction
{
    ChooseTag(TagInputAction),
    PerformRoll(DiceInputAction),
    InputNumber(NumberInputAction),
    SetBool,
    // PickOption(Vec<String>),
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct TagInputAction
{
    restriction: Option<HashSet<Tag>>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct DiceInputAction
{
    dice_to_roll: DiceRoll, // TODO: Ref by tag?
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct NumberInputAction
{
    restriction: Option<NumberInputRestriction>
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum NumberInputRestriction
{
    Min(f32),
    Max(f32),
    Clamped(f32, f32),
}

/// This is the couterpart to InputAction. For each input action,
/// there is a corresponding input response.
#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum InputResponse
{
    ChooseTag(Tag),
    PerformRoll(DiceRollResult),
    InputNumber(f32),
    SetBool(bool),
}