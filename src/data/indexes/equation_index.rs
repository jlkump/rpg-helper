use std::{collections::HashMap, fmt::Display};

use crate::{data::equation::Equation, error::{DefinitionError, InsertionError}};

pub struct EquationIndex {
    equations: HashMap<String, Equation>,
}

impl EquationIndex {
    pub fn new() -> EquationIndexBuilder {
        EquationIndexBuilder { equations: HashMap::new() }
    }

    pub fn get_equation(&mut self, for_type: &str) -> Option<Equation> {
        self.equations.remove(for_type)
    }
}

impl Display for EquationIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (s, e) in &self.equations {
            write!(f, "{:>10}: {}\n", s, e)?
        }
        Ok(())
    }
}

pub struct EquationIndexBuilder {
    equations: HashMap<String, Equation>,
}

impl EquationIndexBuilder {
    pub fn define_equation(mut self, for_type: &str, e: Equation) -> Result<Self, DefinitionError<String>> {
        if self.equations.contains_key(for_type) {
            Err(DefinitionError::Redef(for_type.to_string()))
        } else {
            self.equations.insert(for_type.to_string(), e);
            Ok(self)
        }
    }

    pub fn build(self) -> EquationIndex {
        EquationIndex { equations: self.equations }
    }
}
