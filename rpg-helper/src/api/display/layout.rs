pub mod panel;
pub mod sheet;

pub struct Dimension
{
    width: Unit,
    height: Unit,

    width_clamp: (Option<Unit>, Option<Unit>),  // Optional Min, Max
    height_clamp: (Option<Unit>, Option<Unit>),  // Optional Min, Max
}

pub enum Unit
{
    Px(u32),
    Percent(f32),
    Viewport(f32),
}

pub enum LayoutDirection
{
    Vertical,
    Horizontal,
}