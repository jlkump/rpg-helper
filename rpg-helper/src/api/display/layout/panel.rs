use crate::api::{data::tag::Tag, display::layout::{Dimension, LayoutDirection}};

pub struct Panel
{
    t: PanelType,
    d: Dimension,
    perfered_layout_direction: LayoutDirection,
}

pub enum PanelType
{
    // Wraps what has a tooltip. How the tooltip is displayed is defined as another panel
    Tooltip(Box<Panel>, Box<Panel>), // Child target, tooltip display

    TimelinePanel, // A timeline panel has the option to be filtered for what to display. By default, it shows just the active character's details.
    ValueDisplay,

    // These define a context in which values, abilities, and items can be displayed.
    // For example, the display for all of the characteristics of an Ars Magica character
    // will begin with a QueryValues, targeting "attributes.characteristics"
    // This will return a vector of values. The vector of values will be displayed as
    // defined by a panel which takes in a value, and the tag prefix of that value.
    // The panel may query for additional values (such as for characteristics, the
    // "age points"). These additional values may only be displayed on tool-tip hovers.
    // QueryValueResult
    // - Gives the immediate result tag (ex: "values.characteristics.intelligence")
    // - Gives the numeric value
    QueryValues,
    QueryAbilities,
    QueryItems,
}

// A leaf display node. Simply displays a numeric value derived from an attribute or equation
// There are two options for this display. One is a direct query for the value,
// in which the full path of the tag is given. The other is by providing a tag
// suffix which will be prefixed by the according parent tag context.
// This is used when displaying values from a query values node.
pub enum ValueDisplay
{
    DirectTag(Tag),
    SuffixTag(Tag),
}