use serde::{Deserialize, Serialize};

use crate::api::data::{error::DataError, tag::Tag, Context};

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize, Clone)]
pub enum EvalError
{
    DivideByZero,
    ValueNotFound,
    ExpectedValueMismatch,
    EvaluationMismatch,
}

impl From<EvalError> for DataError
{
    fn from(value: EvalError) -> Self {
        DataError::Evaluation(value)
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct EvalTree
{
    root: EvalNode,
}

impl EvalTree
{
    pub fn eval_as_num(&self, ctx: &Context) -> Result<f32, DataError>
    {
        if self.root.expected_result() != ExpectedValue::Number
        {
            return Err(EvalError::ExpectedValueMismatch.into());
        }
        if let EvalResult::Number(n) = self.root.recursive_eval(ctx)?
        {
            Ok(n)
        }
        else
        {
            Err(EvalError::EvaluationMismatch.into())
        }
    }

    pub fn eval_as_bool(&self, ctx: &Context) -> Result<bool, DataError>
    {
        if self.root.expected_result() != ExpectedValue::Boolean
        {
            return Err(EvalError::ExpectedValueMismatch.into());
        }
        if let EvalResult::Boolean(b) = self.root.recursive_eval(ctx)?
        {
            Ok(b)
        }
        else
        {
            Err(EvalError::EvaluationMismatch.into())
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
enum EvalNode
{
    Operand(OperandNode),
    Operation(OperationNode),
}

fn number_op<F>(
    v1: &Box<EvalNode>, 
    v2: &Box<EvalNode>, 
    ctx: &Context, 
    f: F
) -> Result<EvalResult, DataError>
where
    F: Fn(f32, f32) -> f32
{
    let v1 = v1.recursive_eval(ctx)?.as_number()?;
    let v2 = v2.recursive_eval(ctx)?.as_number()?;
    Ok(EvalResult::Number(f(v1, v2)))
}

fn bool_op<F>(
    v1: &Box<EvalNode>, 
    v2: &Box<EvalNode>, 
    ctx: &Context, 
    f: F
) -> Result<EvalResult, DataError>
where
    F: Fn(EvalResult, EvalResult) -> Result<bool, DataError>
{
    let v1 = v1.recursive_eval(ctx)?;
    let v2 = v2.recursive_eval(ctx)?;
    Ok(EvalResult::Boolean(f(v1, v2)?))
}

impl EvalNode
{
    fn recursive_eval(&self, ctx: &Context) -> Result<EvalResult, DataError>
    {
        match &self
        {
            EvalNode::Operand(operand_node) => 
            match operand_node
            {
                OperandNode::ExplicitNumber(n) => Ok(EvalResult::Number(*n)),
                OperandNode::ExplicitBool(b) => Ok(EvalResult::Boolean(*b)),
                OperandNode::ReferencedValue(tag) => 
                {
                    if let Some(v) = ctx.get_value(tag)?
                    {
                        Ok(EvalResult::Number(v))
                    }
                    else
                    {
                        Err(EvalError::ValueNotFound.into())
                    }
                },
                OperandNode::ReferencedCondition(tag) => Ok(EvalResult::Boolean(ctx.eval_conditional(tag)?)),
            },
            EvalNode::Operation(operation_node) => 
            match operation_node
            {
                OperationNode::Add(v1, v2) => number_op(v1, v2, ctx, |n1, n2| n1 + n2),
                OperationNode::Subtract(v1, v2) => number_op(v1, v2, ctx, |n1, n2| n1 - n2),
                OperationNode::Multiply(v1, v2) => number_op(v1, v2, ctx, |n1, n2| n1 * n2),
                OperationNode::Divide(v1, v2) => number_op(v1, v2, ctx, |n1, n2| n1 / n2), // TODO: Check for divide by zero?
                OperationNode::Negate(v1) => Ok(EvalResult::Number(-v1.recursive_eval(ctx)?.as_number()?)),
                OperationNode::Pow(v1, v2) => number_op(v1, v2, ctx, |n1, n2| n1.powf(n2)),
                OperationNode::Sqrt(v1) => Ok(EvalResult::Number(v1.recursive_eval(ctx)?.as_number()?.sqrt())),
                OperationNode::Round(v1) => Ok(EvalResult::Number(v1.recursive_eval(ctx)?.as_number()?.round())),
                OperationNode::RoundDown(v1) => Ok(EvalResult::Number(v1.recursive_eval(ctx)?.as_number()?.floor())),
                OperationNode::RoundUp(v1) => Ok(EvalResult::Number(v1.recursive_eval(ctx)?.as_number()?.ceil())),
                OperationNode::Range(v1, v2, v3) => Ok(EvalResult::Number(v1.recursive_eval(ctx)?.as_number()?.clamp(v2.recursive_eval(ctx)?.as_number()?, v3.recursive_eval(ctx)?.as_number()?))),

                OperationNode::Equal(v1, v2) => bool_op(v1, v2, ctx, |n1, n2| Ok(n1 == n2)),
                OperationNode::NotEqual(v1, v2) => bool_op(v1, v2, ctx, |n1, n2| Ok(n1 != n2)),
                OperationNode::LessThan(v1, v2) => bool_op(v1, v2, ctx, |n1, n2| Ok(n1.as_number()? < n2.as_number()?)),
                OperationNode::LessThanEq(v1, v2) => bool_op(v1, v2, ctx, |n1, n2| Ok(n1.as_number()? <= n2.as_number()?)),
                OperationNode::GreaterThan(v1, v2) => bool_op(v1, v2, ctx, |n1, n2| Ok(n1.as_number()? > n2.as_number()?)),
                OperationNode::GreaterThanEq(v1, v2) => bool_op(v1, v2, ctx, |n1, n2| Ok(n1.as_number()? >= n2.as_number()?)),
                OperationNode::Not(v1) => Ok(EvalResult::Boolean(!v1.recursive_eval(ctx)?.as_bool()?)),
                OperationNode::Or(v1, v2) => bool_op(v1, v2, ctx, |n1, n2| Ok(n1.as_bool()? || n2.as_bool()?)),
                OperationNode::And(v1, v2) => bool_op(v1, v2, ctx, |n1, n2| Ok(n1.as_bool()? && n2.as_bool()?)),

                OperationNode::Ternary(v1, v2, v3) => 
                {
                    if v1.recursive_eval(ctx)?.as_bool()?
                    {
                        v2.recursive_eval(ctx)
                    }
                    else
                    {
                        v3.recursive_eval(ctx)
                    }
                },
            },
        }
    }

    fn expected_result(&self) -> ExpectedValue
    {
        match &self
        {
            EvalNode::Operand(operand_node) => 
            match operand_node {
                OperandNode::ExplicitNumber(_) | OperandNode::ReferencedValue(_) => ExpectedValue::Number,
                OperandNode::ExplicitBool(_) | OperandNode::ReferencedCondition(_) => ExpectedValue::Boolean,
            },
            EvalNode::Operation(operation_node) =>
            match operation_node {
                OperationNode::Add(_, _) | OperationNode::Subtract(_, _) | OperationNode::Multiply(_, _) | OperationNode::Divide(_, _) | OperationNode::Negate(_) | OperationNode::Pow(_, _) | OperationNode::Sqrt(_) | OperationNode::Round(_) | OperationNode::RoundDown(_) | OperationNode::RoundUp(_) | OperationNode::Range(_, _, _) => ExpectedValue::Number,

                OperationNode::Equal(_, _) | OperationNode::NotEqual(_, _) | OperationNode::LessThan(_, _) | OperationNode::LessThanEq(_, _) | OperationNode::GreaterThan(_, _) | OperationNode::GreaterThanEq(_, _) | OperationNode::Not(_) | OperationNode::Or(_, _) | OperationNode::And(_, _) => ExpectedValue::Boolean,

                OperationNode::Ternary(_, eval_node, _) => eval_node.expected_result(),
            },
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
enum EvalResult
{
    Number(f32),
    Boolean(bool)
}

impl EvalResult
{
    fn as_number(self) -> Result<f32, EvalError>
    {
        match self
        {
            EvalResult::Number(n) => Ok(n),
            EvalResult::Boolean(_) => Err(EvalError::EvaluationMismatch),
        }
    }

    fn as_bool(self) -> Result<bool, EvalError>
    {
        match self
        {
            EvalResult::Number(_) => Err(EvalError::EvaluationMismatch),
            EvalResult::Boolean(b) => Ok(b),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
enum OperandNode
{
    ExplicitNumber(f32),
    ExplicitBool(bool),
    ReferencedValue(Tag),
    ReferencedCondition(Tag),
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
// Operation Nodes act on their children, which may be either an operand node or a operation node
enum OperationNode
{
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
    Range(Box<EvalNode>, Box<EvalNode>, Box<EvalNode>),
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
    // Expects any
    Ternary(Box<EvalNode>, Box<EvalNode>, Box<EvalNode>),
}

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize, Clone)]
pub enum ExpectedValue
{
    Boolean,
    Number,
}