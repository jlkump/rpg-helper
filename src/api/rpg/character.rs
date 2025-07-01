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

use crate::api::data::Context;

pub struct Character
{
    data: Context,
}