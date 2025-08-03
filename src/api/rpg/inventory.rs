use std::{collections::HashMap, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::api::data::tag::Tag;

/// An inventory is associated with a character. It contains
/// a collection of items, which are identified by tag.
/// 
/// The function of an item is similar to an event, in that
/// it can modify a character's stats by layering on the
/// data context conditionally.
/// 
/// An inventory uses specific inventory related attributes
/// to define its important data (number of slots)
/// 
/// A character also has a set of equiped items, which actually
/// apply the changes of the item to the character. The changes
/// of an item only take place if the item's condition evaluates
/// to true.
/// 
/// An items is defined by a set of attributes and data similar 
/// to a character. These attributes and tags of the item are
/// applied to the wearer when the condition evaluates true.
#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Inventory
{
    items: Vec<Item>,          // Stored items
    slots: HashMap<Tag, Item>, // Equiped items
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Item
{
    id: Tag,
    spec: Tag,
    count: u32,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct ItemSpec
{
    id: Tag,
    id_prefix: Tag,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct ItemSet
{

}