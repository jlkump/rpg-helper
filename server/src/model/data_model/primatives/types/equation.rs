use serde::{Deserialize, Serialize};

use crate::model::data_model::{primatives::{input::{Input, InputRequest}, values::Value}, storage::types::{EquationRef, TypeRef}};

use super::Type;

// Results
// Ok -> EvalResult
// Err -> EvalError
pub enum EvalResult<T> {
    Value(T),
    InputRequired(Vec<InputRequest>),
}

pub enum EvalError {
    DivideByZero,
    NonExistantNamedValue,
    GotWrongType(TypeRef)
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct Equation {
    name: String,
    ast: EvalTree,
}

/// EquationCompute is handle to re-try a given equation with the correct inputs.
#[derive(Debug)]
pub struct EquationCompute {
    t: EquationRef,
    inputs: Vec<Input>,
}

impl EquationCompute {
    pub fn set_inputs(&mut self, inputs: Vec<Input>) {
        // Error check correct number of inputs?
        self.inputs = inputs;
    }

    fn eval_f32(&self) -> Result<EvalResult<f32>, EvalError> {
        todo!()
    }

    fn eval_bool(&self) -> Result<EvalResult<bool>, EvalError> {
        todo!()
    }
}

impl Equation {
    pub fn from_str(s: &str, equation_name: &str) -> Equation {
        todo!()
    }

    pub fn compute(&self) -> EquationCompute {
        EquationCompute {
            t: todo!(),
            inputs: vec![],
        }
    }

    fn ast_compute(&self) -> Result<EvalResult<f32>, EvalError> {
        todo!()
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
struct EvalNode {
    // TODO: Probably want to define in a separate file for Eval Tree. Will be pretty large
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct EvalTree {
    root: EvalNode,
}

impl EvalNode {
    fn eval_expect(&self, expected: &Type) -> Result<EvalResult<Value>, EvalError> {
        todo!()
    }
}