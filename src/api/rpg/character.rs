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

use crate::api::data::{tag::Tag, context::Context};

// First todo:
//      1. Parse json in order to import character data
//      Example:
//
//      {
//          "ruleset": {
//              "id": "",
//              "version": "1.0.0"
//          },
//          "text_data": [{"name", "Test Character"}, {"name.alias.0": "First Character"}],
//          "data":
//          {
//              "state_tags": ["tag", "tag.two"],
//              "attributes": [{"name": "attribute.name", "value": "0.0"}],
//              "modifiers": [{"name": "modifier.name", "target": "attribute.name", "condition": "condition.name", "change": "3.0"}],
//              "equations": [{"name": "equation.name", "equation": "attribute.name + 3.0"}],
//              "conditionals": [{"name": "conditional.name", "conditional": "equation.name == 3.0"}],
//          },
//          "timeline": [{"event_name": "event_temp_name", "date": ...}]
//      }    

pub struct Character
{
    data: Context,
    text_data: HashMap<Tag, String>,
}