use crate::api::data::{tag::Tag, Context};


#[derive(Debug, Deserialize, PartialEq, Eq, Serialize, Clone)]
pub enum EvalError
{
    DivideByZero,
    ValueNotFound,
    ExpectedValueMismatch,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct EvalTree
{
    root: EvalNode,
}

impl EvalTree
{
    pub fn eval_as_num(&self, ctx: &Context) -> Result<f32, EvalError>
    {
        if self.root.expected_result() != ExpectedValue::Number
        {
            return Err(EvalError::ExpectedValueMismatch);
        }
        todo!()
    }

    pub fn eval_as_bool(&self, ctx: &Context) -> Result<bool, EvalError>
    {
        if self.root.expected_result() != ExpectedValue::Boolean
        {
            return Err(EvalError::ExpectedValueMismatch);
        }
        todo!()
    }
}

enum EvalNode
{
    Operand(OperandNode),
    Operation(OperationNode),
}

impl EvalNode
{
    fn recursive_eval_num(&self, ctx: &Context) -> Result<f32, EvalError>
    {
        todo!()
    }

    fn recursive_eval_bool(&self, ctx: &Context) -> Result<bool, EvalError>
    {
        todo!()
    }

    fn expected_result(&self) -> ExpectedValue
    {
        todo!()
    }
}

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