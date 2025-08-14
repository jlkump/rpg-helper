use std::ops;

use self::{equation_index::EquationIndex, type_index::TypeIndex, value_index::ValueIndex};

use super::timeline::Timeline;

pub mod equation_index;
pub mod type_index;
pub mod value_index;


// An Index holds all the information for the Ruleset, the Setting, and the Characters.
// A Ruleset defines the types and default values, the generic events available, and the equations
// available to characters. A Setting can add or modify these values, events and equations. A Character
// is the leaf of this heirarchy, holding the values and events to display to the player. They may make their own
// types or equations, but most often they will use the ones provided by the Ruleset or Setting Indices.
pub struct Index<'a> {
    id: u32,
    values: ValueIndex<'a>,
    types: TypeIndex,
    equations: EquationIndex, // Equations are held by ValueTy
    timeline: Timeline, // Timeline holds the events
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

impl<'a> ops::Add<Index<'a>> for Index<'a> {
    type Output = Index<'a>;

    fn add(self, _rhs: Index) -> Index<'a> {
        // Gives an index which holds the combination of the two indices.
        // Values that are duplicated are overridden by the LHS Index.
        todo!()
    }
}

impl PartialEq for Index<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}