use serde::{Deserialize, Serialize};

use crate::model::data_model::{primatives::{input::{Input, InputRequest}, values::Value}, storage::{types::{EquationRef, TypeRef}, ContainerKind, IndexRef, Query, RefTarget, Storable}};

use super::Type;

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum EvalError
{
    DivideByZero,
    TypeMismatch(TypeRef),
    TypeNotFound,
    ValueNotFound,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct Equation {
    name: String,
    container: ContainerKind,
    expects: TypeRef, 
    ast: EvalTree,
}

impl Storable for Equation {
    fn get_container(&self) -> &ContainerKind {
        &self.container
    }
}

/// EquationCompute is handle to re-try a given equation with the correct inputs.
#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct EquationCompute {
    t: EquationRef,
    inputs: Vec<Input>, // Input Request could be any type
}

impl EquationCompute {
    pub fn set_inputs(&mut self, inputs: Vec<Input>) {
        // Error check correct number of inputs?
        self.inputs = inputs;
    }

    pub fn get_req_inputs(&self) -> Vec<InputRequest> {
        todo!()
    }

    pub fn as_number(&self) -> Query<f32> {
        todo!()
    }

    pub fn as_bool(&self) -> Query<bool> {
        todo!()
    }

    pub fn as_value(&self) -> Query<Value> {
        todo!()
    }
}

impl Equation {
    pub fn from_str(s: &str, equation_name: &str) -> Equation {
        todo!()
    }

    pub fn eval(&self, inputs: Option<Vec<Input>>) -> EquationCompute {
        EquationCompute {
            t: todo!(),
            inputs: vec![],
        }
    }

    fn ast_compute(&self) -> Query<Value> {
        todo!()
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
struct EvalNode {
    // TODO: Probably want to define in a separate file for Eval Tree. Will be pretty large
}

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
    Pow,
    Sqrt,
    Round,
    RoundDown,
    RoundUp,
    Ternary,
    Query,
    Find,
    Equal,
    NotEqual,
    LessThan,
    LessThanEq,
    GreaterThan,
    GreaterThanEq,
    Not,
    Or,
    And,
}

enum ExpectedValue {
    Number,
    Boolean,
    MetaTypeWithField(String),
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct EvalTree {
    root: EvalNode,
}

impl EvalNode {
    fn eval_expect(&self, expected: &Type) -> Query<Value> {
        todo!()
    }
}