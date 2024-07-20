use serde::{Deserialize, Serialize};

use crate::model::data_model::{primatives::{input::{Input, InputRequest}, values::{boolean::Bool, die_roll::DieRoll, number::Number, Value}}, storage::{types::{DieRollTypeRef, EquationRef, TypeRef}, values::ValueRef, view_context::ViewContext, ContainerKind, IndexRef, Query, QueryError, RefTarget, Storable}};

use super::{die_roll::DieRollType, Type};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum EvalError
{
    DivideByZero,
    RequiresInput,
    RequiresDieRoll,
    TypeNotFound(String),
    ValueNotFound(String),
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Equation {
    name: String,
    equation_string: String,  // This gets converted to the AST, but it probably doesn't hurt to store it as well.
    self_ref: EquationRef,
    expects: ExpectedValue, 
    ast: EvalTree,
}

impl Storable for Equation {
    fn get_container(&self) -> &ContainerKind {
        &self.self_ref.get_container()
    }
}

impl Equation {
    pub fn from_str(s: &str, equation_name: &str, container: ContainerKind) -> Equation {
        let ast = EvalTree::from_str(s);
        let expects = ast.expected_value();
        Equation { 
            name: equation_name.to_string(), 
            equation_string: s.to_string(), 
            self_ref: EquationRef::new(equation_name, container), 
            expects, 
            ast, 
        }
    }

    pub fn eval(&self) -> EquationCompute {
        EquationCompute {
            t: self.self_ref.clone(),
            inputs: vec![],
        }
    }

    fn ast_compute(&self, context: &ViewContext) -> Query<Value> {
        match self.ast.eval(context) {
            Ok(v) => return Ok(v),
            Err(e) => match e {
                QueryError::Eval(e) => {
                    match e {
                        EvalError::RequiresInput | EvalError::RequiresDieRoll => return Err(QueryError::Input(self.eval())),
                        _ => Err(QueryError::Eval(e))
                    }
                },
                _ => Err(e)
            },
        }
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

    pub fn as_number(&self, context: &ViewContext) -> Query<f32> {
        Ok(self.t.to_ref(context)?.ast_compute(context)?.as_number(context)?)
    }

    pub fn as_bool(&self, context: &ViewContext) -> Query<bool> {
        Ok(self.t.to_ref(context)?.ast_compute(context)?.as_bool(context)?)
    }

    pub fn as_value(&self, context: &ViewContext) -> Query<Value> {
        self.t.to_ref(context)?.ast_compute(context)
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
struct EvalTree {
    root: EvalNode,
}

impl EvalTree {
    
    fn from_str(s: &str) -> EvalTree {
        // Tokenize string
        // Return err if syntax error exists
        todo!()
    }
    
    fn expected_value(&self) -> ExpectedValue {
        self.root.expected_value()
    }

    fn eval(&self, context: &ViewContext) -> Query<Value> {
        self.root.recursive_eval(context)
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
enum EvalNode {
    Operand(OperandNode),
    Operation(OperationNode),
}

fn number_op<F>(v1: &Box<EvalNode>, v2: &Box<EvalNode>, context: &ViewContext, f: F) -> Query<Value> 
where
    F: Fn(f32, f32) -> f32
{
    let v1 = v1.recursive_eval(context)?.as_number(context)?;
    let v2 = v2.recursive_eval(context)?.as_number(context)?;
    Ok(Value::Num(Number::generic(f(v1, v2))))
}

fn boolean_op<F>(v1: &Box<EvalNode>, v2: &Box<EvalNode>, context: &ViewContext, f: F) -> Query<Value> 
where
    F: Fn(Value, Value) -> bool
{
    let v1 = v1.recursive_eval(context)?;
    let v2 = v2.recursive_eval(context)?;
    Ok(Value::Bool(Bool::generic(f(v1, v2))))
}

impl EvalNode {
    fn recursive_eval(&self, context: &ViewContext) -> Query<Value> {
        match self {
            EvalNode::Operand(o) => {
                match o {
                    OperandNode::Number(n) => Ok(Value::Num(Number::generic(*n))),
                    OperandNode::Boolean(b) => Ok(Value::Bool(Bool::generic(*b))),
                    OperandNode::ValueRef(v) => Ok(v.to_ref(context)?.clone()),
                    OperandNode::DieRoll(_) => Err(QueryError::Eval(EvalError::RequiresDieRoll)),
                    OperandNode::Input(_) => Err(QueryError::Eval(EvalError::RequiresInput)),
                }
            },
            EvalNode::Operation(op) => {
                match op {
                    OperationNode::Add(v1, v2) => 
                        number_op(v1, v2, context, |n1, n2| n1 + n2),
                    OperationNode::Subtract(v1, v2) => 
                        number_op(v1, v2, context, |n1, n2| n1 - n2),
                    OperationNode::Multiply(v1, v2) => 
                        number_op(v1, v2, context, |n1, n2| n1 * n2),
                    OperationNode::Divide(v1, v2) =>
                        number_op(v1, v2, context, |n1, n2| n1 / n2),
                    OperationNode::Negate(v1) => 
                        Ok(Value::Num(Number::generic(-v1.recursive_eval(context)?.as_number(context)?))),
                    OperationNode::Pow(v1, v2) =>
                        number_op(v1, v2, context, |n1, n2| n1.powf(n2)),
                    OperationNode::Sqrt(v1) =>
                        Ok(Value::Num(Number::generic(v1.recursive_eval(context)?.as_number(context)?.sqrt()))),
                    OperationNode::Round(v1) => 
                        Ok(Value::Num(Number::generic(v1.recursive_eval(context)?.as_number(context)?.round()))),
                    OperationNode::RoundDown(v1) =>
                        Ok(Value::Num(Number::generic(v1.recursive_eval(context)?.as_number(context)?.floor()))),
                    OperationNode::RoundUp(v1) =>
                        Ok(Value::Num(Number::generic(v1.recursive_eval(context)?.as_number(context)?.ceil()))),
                    OperationNode::Equal(v1, v2) =>
                        boolean_op(v1, v2, context, |n1, n2| n1.eq(&n2)),
                    OperationNode::NotEqual(v1, v2) => 
                        boolean_op(v1, v2, context, |n1, n2| n1.ne(&n2)),
                    OperationNode::LessThan(v1, v2) => {
                        let v1 = v1.recursive_eval(context)?.as_number(context)?;
                        let v2 = v2.recursive_eval(context)?.as_number(context)?;
                        Ok(Value::Bool(Bool::generic(v1 < v2)))
                    },
                    OperationNode::LessThanEq(v1, v2) => {
                        let v1 = v1.recursive_eval(context)?.as_number(context)?;
                        let v2 = v2.recursive_eval(context)?.as_number(context)?;
                        Ok(Value::Bool(Bool::generic(v1 <= v2)))
                    },
                    OperationNode::GreaterThan(v1, v2) => {
                        let v1 = v1.recursive_eval(context)?.as_number(context)?;
                        let v2 = v2.recursive_eval(context)?.as_number(context)?;
                        Ok(Value::Bool(Bool::generic(v1 > v2)))
                    },
                    OperationNode::GreaterThanEq(v1, v2) => {
                        let v1 = v1.recursive_eval(context)?.as_number(context)?;
                        let v2 = v2.recursive_eval(context)?.as_number(context)?;
                        Ok(Value::Bool(Bool::generic(v1 >= v2)))
                    },
                    OperationNode::Not(v1) =>
                        Ok(Value::Bool(Bool::generic(!v1.recursive_eval(context)?.as_bool(context)?))),
                    OperationNode::Or(v1, v2) => {
                        let v1 = v1.recursive_eval(context)?.as_bool(context)?;
                        let v2 = v2.recursive_eval(context)?.as_bool(context)?;
                        Ok(Value::Bool(Bool::generic(v1 || v2)))
                    },
                    OperationNode::And(v1, v2) => {
                        let v1 = v1.recursive_eval(context)?.as_bool(context)?;
                        let v2 = v2.recursive_eval(context)?.as_bool(context)?;
                        Ok(Value::Bool(Bool::generic(v1 && v2)))
                    },
                    OperationNode::Range(v1, min, max) => {
                        Ok(Value::Num(Number::generic(v1.recursive_eval(context)?.as_number(context)?
                            .clamp(
                                min.recursive_eval(context)?.as_number(context)?, 
                                max.recursive_eval(context)?.as_number(context)?
                            )
                        )))
                    },
                    OperationNode::Ternary(v1, v2, v3) => {
                        if v1.recursive_eval(context)?.as_bool(context)? {
                            v2.recursive_eval(context)
                        } else {
                            v3.recursive_eval(context)
                        }
                    }
                }
            },
        }
    }
}

impl EvalNode {
    fn expected_value(&self) -> ExpectedValue {
        match &self {
            EvalNode::Operand(o) => o.expected_value(),
            EvalNode::Operation(o) => o.expected_value(),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
// Operand Nodes are the leaf nodes of a AST
enum OperandNode {
    Number(f32),
    Boolean(bool),
    ValueRef(ValueRef),
    DieRoll(DieRollTypeRef),
    Input(InputRequest),
}

impl OperandNode {
    fn expected_value(&self) -> ExpectedValue {
        match &self {
            OperandNode::Number(_) => ExpectedValue::Number,
            OperandNode::Boolean(_) => ExpectedValue::Boolean,
            OperandNode::ValueRef(_) => ExpectedValue::Value,
            OperandNode::DieRoll(_) => ExpectedValue::Number,
            OperandNode::Input(_) => ExpectedValue::Value,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
// Operation Nodes act on their children, which may be either an operand node or a operation node
enum OperationNode {
    // Expects numeric result
    Add(Box<EvalNode>, Box<EvalNode>),
    Subtract(Box<EvalNode>, Box<EvalNode>),
    Multiply(Box<EvalNode>, Box<EvalNode>),
    Divide(Box<EvalNode>, Box<EvalNode>),
    Negate(Box<EvalNode>),
    Pow(Box<EvalNode>, Box<EvalNode>),
    Sqrt(Box<EvalNode>),
    Round(Box<EvalNode>),
    RoundDown(Box<EvalNode>),
    RoundUp(Box<EvalNode>),
    // Expects boolean result
    Equal(Box<EvalNode>, Box<EvalNode>),
    NotEqual(Box<EvalNode>, Box<EvalNode>),
    LessThan(Box<EvalNode>, Box<EvalNode>),
    LessThanEq(Box<EvalNode>, Box<EvalNode>),
    GreaterThan(Box<EvalNode>, Box<EvalNode>),
    GreaterThanEq(Box<EvalNode>, Box<EvalNode>),
    Not(Box<EvalNode>),
    Or(Box<EvalNode>, Box<EvalNode>),
    And(Box<EvalNode>, Box<EvalNode>),
    Range(Box<EvalNode>, Box<EvalNode>, Box<EvalNode>), // Value, min, max
    // Expects any
    Ternary(Box<EvalNode>, Box<EvalNode>, Box<EvalNode>),
    // Field and list queries can be stored in the ValueRef itself instead
    // FieldQuery(Box<EvalNode>, String), // Points to meta-inst, given string is field name
    // ListQuery(Box<EvalNode>, usize), // Points to list, given number is index
}

impl OperationNode {
    fn expected_value(&self) -> ExpectedValue {
        match self {
            OperationNode::Add(_, _) => ExpectedValue::Number,
            OperationNode::Subtract(_, _) => ExpectedValue::Number,
            OperationNode::Multiply(_, _) => ExpectedValue::Number,
            OperationNode::Divide(_, _) => ExpectedValue::Number,
            OperationNode::Negate(_) => ExpectedValue::Number,
            OperationNode::Pow(_, _) => ExpectedValue::Number,
            OperationNode::Sqrt(_) => ExpectedValue::Number,
            OperationNode::Round(_) => ExpectedValue::Number,
            OperationNode::RoundDown(_) => ExpectedValue::Number,
            OperationNode::RoundUp(_) => ExpectedValue::Number,
            OperationNode::Equal(_, _) => ExpectedValue::Boolean,
            OperationNode::NotEqual(_, _) => ExpectedValue::Boolean,
            OperationNode::LessThan(_, _) => ExpectedValue::Boolean,
            OperationNode::LessThanEq(_, _) => ExpectedValue::Boolean,
            OperationNode::GreaterThan(_, _) => ExpectedValue::Boolean,
            OperationNode::GreaterThanEq(_, _) => ExpectedValue::Boolean,
            OperationNode::Not(_) => ExpectedValue::Boolean,
            OperationNode::Or(_, _) => ExpectedValue::Boolean,
            OperationNode::And(_, _) => ExpectedValue::Boolean,
            OperationNode::Range(_, _, _) => ExpectedValue::Number,
            OperationNode::Ternary(_, _, _) => ExpectedValue::Value,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub enum ExpectedValue {
    Number,
    Boolean,
    Value, // Value could be a number or boolean, so a check must be performed to see if value is a bool or number
}