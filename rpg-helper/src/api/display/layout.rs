use serde::{Deserialize, Serialize};

pub mod panel;
pub mod sheet;

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Dimension
{
    width: Unit,
    height: Unit,

    width_clamp: (Option<Unit>, Option<Unit>),  // Optional Min, Max
    height_clamp: (Option<Unit>, Option<Unit>),  // Optional Min, Max
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum Unit
{
    Px(u32),
    Percent(f32),
    Viewport(f32),
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum LayoutDirection
{
    Vertical,
    Horizontal,
}