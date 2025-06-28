use serde::{Deserialize, Serialize};

use crate::api::data::{error::{DataError, ParseError}, tag::Tag, Context};

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
        if self.can_eval_as_number()
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
        if self.can_eval_as_bool()
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

    pub fn can_eval_as_bool(&self) -> bool
    {
        self.root.expected_result() != ExpectedResult::Number
    }

    pub fn can_eval_as_number(&self) -> bool
    {
        self.root.expected_result() != ExpectedResult::Boolean
    }

    /// Constructs a full abstract syntax tree from the given string.
    /// The syntax for an equation is as follows:
    ///     "3 + 4 * 10 / 5"
    ///     "rounddown((sqrt(8 * Ability.Magic Theory.Exp / 5 + 1)-1)/2)"
    pub fn from_str(s: &str) -> Result<Self, ParseError>
    {
        todo!()
    }

    /// Using the constructed AST, reverses back to the equation form.
    /// The resultant equation uses the minimum required parentheses
    /// with some perfered syntax formatting for some operations 
    /// (such as power of using pow() over #^#).
    pub fn to_string(&self) -> String
    {
        todo!()
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
enum EvalNode
{
    Operand(OperandNode),
    Operation(OperationNode),
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
                OperandNode::ReferencedTag(tag) => 
                            {
                                if ctx.has_conditional(tag)
                                {
                                    Ok(EvalResult::Boolean(ctx.eval_conditional(tag)?))
                                }
                                else if let Some(v) = ctx.get_value(tag)?
                                {
                                    Ok(EvalResult::Number(v))
                                }
                                else
                                {
                                    Err(EvalError::ValueNotFound.into())
                                }

                            },
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

    fn expected_result(&self) -> ExpectedResult
    {
        match &self
        {
            EvalNode::Operand(operand_node) => 
            match operand_node {
                OperandNode::ExplicitNumber(_) | OperandNode::ReferencedValue(_) => ExpectedResult::Number,
                OperandNode::ExplicitBool(_) | OperandNode::ReferencedCondition(_) => ExpectedResult::Boolean,
                OperandNode::ReferencedTag(_) => ExpectedResult::Unknown,
            },
            EvalNode::Operation(operation_node) =>
            match operation_node {
                OperationNode::Add(_, _) | OperationNode::Subtract(_, _) | OperationNode::Multiply(_, _) | OperationNode::Divide(_, _) | OperationNode::Negate(_) | OperationNode::Pow(_, _) | OperationNode::Sqrt(_) | OperationNode::Round(_) | OperationNode::RoundDown(_) | OperationNode::RoundUp(_) | OperationNode::Range(_, _, _) => ExpectedResult::Number,

                OperationNode::Equal(_, _) | OperationNode::NotEqual(_, _) | OperationNode::LessThan(_, _) | OperationNode::LessThanEq(_, _) | OperationNode::GreaterThan(_, _) | OperationNode::GreaterThanEq(_, _) | OperationNode::Not(_) | OperationNode::Or(_, _) | OperationNode::And(_, _) => ExpectedResult::Boolean,

                // While a ternary could return a boolean, we just prevent that use-case as you should just use boolean operators instead.
                OperationNode::Ternary(_, _, _) => ExpectedResult::Number,
            },
        }
    }
}

fn number_op<F>(v1: &Box<EvalNode>, v2: &Box<EvalNode>, ctx: &Context, f: F) -> Result<EvalResult, DataError>
where
    F: Fn(f32, f32) -> f32
{
    let v1 = v1.recursive_eval(ctx)?.as_number()?;
    let v2 = v2.recursive_eval(ctx)?.as_number()?;
    Ok(EvalResult::Number(f(v1, v2)))
}

fn bool_op<F>(v1: &Box<EvalNode>, v2: &Box<EvalNode>, ctx: &Context, f: F) -> Result<EvalResult, DataError>
where
    F: Fn(EvalResult, EvalResult) -> Result<bool, DataError>
{
    let v1 = v1.recursive_eval(ctx)?;
    let v2 = v2.recursive_eval(ctx)?;
    Ok(EvalResult::Boolean(f(v1, v2)?))
}

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize, Clone)]
enum ExpectedResult
{
    Boolean,
    Number,
    Unknown,
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
    // The type of tag reference is determined by the expected value requested.
    // If the value can not be determined, we fallback to ReferencedTag
    ReferencedValue(Tag),
    ReferencedCondition(Tag),
    ReferencedTag(Tag),
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
    Ternary(Box<EvalNode>, Box<EvalNode>, Box<EvalNode>),
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
}

pub(super) mod tokenize
{
    use crate::api::data::{error::{EvalParseError, ParseError, ParseErrorType}, tag::Tag};

    pub(super) enum Token
    {
        Tag(Tag),
        OpenParen,
        ClosedParen,
        Operation(String),
        Method(String),
        Number(f32),
        Bool(bool),
    }

    /// Splits a given string into tokens
    /// so they can be more easily parsed into
    /// a ast.
    pub(super) fn tokenize_expression(s: &str) -> Result<Vec<Token>, ParseError>
    {
        let mut res = vec![];
        let mut s = remove_unneeded_whitespace(s);
        while let Some((remaining, token)) = get_next_token(s)?
        {
            s = remaining;
            if let Some(token) = token
            {
                res.push(token);
            }
        }
        Ok(res)
    }

    fn get_next_token(mut s: String) -> Result<Option<(String, Option<Token>)>, ParseError>
    {
        if s.is_empty() || s.chars().all(char::is_whitespace)
        {
            Ok(None)
        }
        else
        {
            let last_ind = s.find(|c: char| !c.is_alphanumeric() && c != '.').unwrap_or(s.len() - 1);
            let remaining = s.split_off(last_ind);
            if s.is_empty() || s.chars().all(char::is_whitespace)
            {
                Ok(Some((remaining, None)))
            }
            else
            {
                let token = str_to_token(&s)?;
                Ok(Some((remaining, Some(token))))
            }
        }
    }
    
    fn str_to_token(ts: &str) -> Result<Token, ParseError>
    {
        let ts = ts.trim();
        if let Ok(num) = ts.parse()
        {
            Ok(Token::Number(num))
        }
        else if let Ok(b) = ts.parse()
        {
            Ok(Token::Bool(b))
        }
        else if ts.chars().nth(0) == Some('(')
        {
            Ok(Token::OpenParen)
        }
        else if ts.chars().nth(0) == Some(')')
        {
            Ok(Token::ClosedParen)
        }
        else if ts == "+"|| ts == "-" || ts == "==" 
        {
            Ok(Token::Operation(ts.to_string()))
        }
        else if ts == "rounddown"
        {
            Ok(Token::Operation(ts.to_string()))
        }
        else if let Ok(tag) = Tag::from_str(ts)
        {
            Ok(Token::Tag(tag))
        }
        else
        {
            Err(ParseError::new(ts.to_string(), 0, ParseErrorType::Evaluation(EvalParseError::TokenInvalid)))
        }
    }

    /// All whitespace that is not part of a tag
    /// should be removed to make tokenization easier
    fn remove_unneeded_whitespace(s: &str) -> String
    {
        let mut result = String::new();
        let mut previous: Option<char> = None;

        for (index, c) in s.chars().enumerate() {
            if c.is_whitespace() {
                // Only include normal spaces in tags 
                if c == ' ' {
                    // Only include if the previous char was an alpha or '.' and next character is a '.' or alpha
                    if let Some(prev) = previous {
                        let mut i = index;
                        let mut next = s.chars().nth(i);
                        while next.is_some_and(|c| c == ' ') {
                            i = i + 1;
                            next = s.chars().nth(i);
                        }

                        if next.is_some_and(|next| (next.is_alphabetic() || next == '.') && (prev.is_alphabetic() || prev == '.')) {
                            result.push(' ');
                        }
                    }
                }
            } else {
                previous = Some(c);
                result.push(c);
            }
        }
        result
    }

    #[cfg(test)]
    mod unit_tests
    {
        use crate::api::data::evaltree::tokenize::remove_unneeded_whitespace;

        #[test]
        fn whitespace_test_1()
        {
            assert_eq!(remove_unneeded_whitespace(" rounddown ( 500 5) "), "rounddown(5005)")
        }

        #[test]
        fn whitespace_test_2()
        {
            assert_eq!(remove_unneeded_whitespace(" rounddown ( test . tag ) "), "rounddown(test . tag)")
        }

        #[test]
        fn whitespace_test_3()
        {
            assert_eq!(remove_unneeded_whitespace(" round down ( test . tag ) "), "round down(test . tag)")
        }
    }
}