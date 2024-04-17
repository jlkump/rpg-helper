use self::{equation_index::EquationIndex, type_index::TypeIndex, value_index::ValueIndex};

use super::timeline::Timeline;

pub mod equation_index;
pub mod type_index;
pub mod value_index;


pub struct Index<'a> {
    id: u32,
    values: ValueIndex<'a>,
    types: TypeIndex, // Reference to the GM's TypeIndex?
    // equations: EquationIndex, // Equations are held by ValueTy
    timeline: Timeline,
    // TODO:
    // 1. Needs to manage what is shared and what isn't for equations, types, values, and events
    // 2. Needs a modifier index for values or equations in specific situations
}

impl<'b> Index<'b> {
    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_values<'a>(&'a self) -> &'a ValueIndex<'b> {
        &self.values
    }
}