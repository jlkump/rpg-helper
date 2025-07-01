use serde::{Deserialize, Serialize};

use crate::api::data::{error::{DataError, TokenizationError}, evaltree::{parse::remove_parentheses, tokenize::Token}, tag::Tag, Context};

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize, Clone)]
pub enum EvalError
{
    DivideByZero,
    ValueNotFound,
    ExpectedValueMismatch,
    EvaluationMismatch,
    UnsupportedOperation,
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
        if !self.can_eval_as_number()
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
        if !self.can_eval_as_bool()
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
    pub fn from_str(s: &str) -> Result<Self, DataError>
    {
        parse::brackets_are_balanced(s)?;

        Ok(EvalTree
        {
            root: EvalNode::from_token_list(tokenize::tokenize_expression(s)?)?,
        })
    }

    /// Using the constructed AST, reverses back to the equation form.
    /// The resultant equation uses the minimum required parentheses
    /// with some perfered syntax formatting for some operations 
    /// (such as power of using pow() over #^#).
    pub fn to_expression_str(&self) -> String
    {
        todo!()
    }
}

impl std::fmt::Display for EvalTree
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) ->  std::fmt::Result
    {
        let mut result = String::new();
        tree_recursive_display_helper(&mut result, &"".to_owned(), &self.root, true);
        write!(f, "{}", result)
    }
}

fn tree_recursive_display_helper(result: &mut String, prefix: &String, node: &EvalNode, end: bool) {
    result.push_str(prefix);
    result.push_str("|__");
    result.push_str(&node.to_string());
    result.push_str("\n");

    match node {
        EvalNode::Operation(node) => {
            let last = node.get_operation().get_number_of_operands() - 1;
            for (i, n) in node.get_children().iter().enumerate() {
                let mut new_prefix = prefix.clone();
                if end
                {
                    new_prefix.push_str("   ");
                }
                else
                {
                    new_prefix.push_str("|  ");
                }
                tree_recursive_display_helper(result, &new_prefix, n, i == last);
            }
        },
        EvalNode::Operand(_) => (),
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

impl OperationNode
{
    fn new(op: Operation, mut children: Vec<EvalNode>) -> OperationNode
    {
        match op
        {
            Operation::Add => OperationNode::Add(Box::new(children.remove(0)), Box::new(children.remove(0))),
            Operation::Subtract => OperationNode::Subtract(Box::new(children.remove(0)), Box::new(children.remove(0))),
            Operation::Multiply => OperationNode::Multiply(Box::new(children.remove(0)), Box::new(children.remove(0))),
            Operation::Divide => OperationNode::Divide(Box::new(children.remove(0)), Box::new(children.remove(0))),
            Operation::Negate => OperationNode::Negate(Box::new(children.remove(0))),
            Operation::PowSymbol => OperationNode::Pow(Box::new(children.remove(0)), Box::new(children.remove(0))),
            Operation::PowMethod => OperationNode::Pow(Box::new(children.remove(0)), Box::new(children.remove(0))),
            Operation::Sqrt => OperationNode::Sqrt(Box::new(children.remove(0))),
            Operation::Round => OperationNode::Round(Box::new(children.remove(0))),
            Operation::RoundDown => OperationNode::RoundDown(Box::new(children.remove(0))),
            Operation::RoundUp => OperationNode::RoundUp(Box::new(children.remove(0))),
            Operation::Range => OperationNode::Range(Box::new(children.remove(0)), Box::new(children.remove(0)), Box::new(children.remove(0))),
            Operation::Ternary => OperationNode::Ternary(Box::new(children.remove(0)), Box::new(children.remove(0)), Box::new(children.remove(0))),
            Operation::Equal => OperationNode::Equal(Box::new(children.remove(0)), Box::new(children.remove(0))),
            Operation::NotEqual => OperationNode::NotEqual(Box::new(children.remove(0)), Box::new(children.remove(0))),
            Operation::LessThan => OperationNode::LessThan(Box::new(children.remove(0)), Box::new(children.remove(0))),
            Operation::LessThanEq => OperationNode::LessThanEq(Box::new(children.remove(0)), Box::new(children.remove(0))),
            Operation::GreaterThan => OperationNode::GreaterThan(Box::new(children.remove(0)), Box::new(children.remove(0))),
            Operation::GreaterThanEq => OperationNode::GreaterThanEq(Box::new(children.remove(0)), Box::new(children.remove(0))),
            Operation::Not => OperationNode::Not(Box::new(children.remove(0))),
            Operation::Or => OperationNode::Or(Box::new(children.remove(0)), Box::new(children.remove(0))),
            Operation::And => OperationNode::And(Box::new(children.remove(0)), Box::new(children.remove(0))),
        }
    }

    fn get_operation(&self) -> Operation
    {
        match self
        {
            OperationNode::Add(_, _) => Operation::Add,
            OperationNode::Subtract(_, _) => Operation::Subtract,
            OperationNode::Multiply(_, _) => Operation::Multiply,
            OperationNode::Divide(_, _) => Operation::Divide,
            OperationNode::Negate(_) => Operation::Negate,
            OperationNode::Pow(_, _) => Operation::PowMethod,
            OperationNode::Sqrt(_) => Operation::Sqrt,
            OperationNode::Round(_) => Operation::Round,
            OperationNode::RoundDown(_) => Operation::RoundDown,
            OperationNode::RoundUp(_) => Operation::RoundUp,
            OperationNode::Range(_, _, _) => Operation::Range,
            OperationNode::Ternary(_, _, _) => Operation::Ternary,
            OperationNode::Equal(_, _) => Operation::Equal,
            OperationNode::NotEqual(_, _) => Operation::NotEqual,
            OperationNode::LessThan(_, _) => Operation::LessThan,
            OperationNode::LessThanEq(_, _) => Operation::LessThanEq,
            OperationNode::GreaterThan(_, _) => Operation::GreaterThan,
            OperationNode::GreaterThanEq(_, _) => Operation::GreaterThanEq,
            OperationNode::Not(_) => Operation::Not,
            OperationNode::Or(_, _) => Operation::Or,
            OperationNode::And(_, _) => Operation::And,
        }
    }

    fn get_children(&self) -> Vec<&Box<EvalNode>>
    {
        match self
        {
            OperationNode::Add(n, n1) | OperationNode::Subtract(n, n1) | OperationNode::Multiply(n, n1) | OperationNode::Divide(n, n1) | OperationNode::Pow(n, n1) |
            OperationNode::Equal(n, n1) | OperationNode::NotEqual(n, n1) | OperationNode::LessThan(n, n1) | OperationNode::LessThanEq(n, n1) | OperationNode::GreaterThan(n, n1) | OperationNode::GreaterThanEq(n, n1) | OperationNode::Or(n, n1) | OperationNode::And(n, n1) => vec![n, n1],
            OperationNode::Negate(n) | OperationNode::Sqrt(n) | OperationNode::Round(n) | OperationNode::RoundDown(n) | OperationNode::RoundUp(n) | OperationNode::Not(n) => vec![n],
            OperationNode::Range(n, n1, n2) | OperationNode::Ternary(n, n1, n2) => vec![n, n1, n2],
        }
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
    fn from_token_list(tokens: Vec<Token>) -> Result<EvalNode, DataError>
    {
        Self::build_node(tokens, ExpectedResult::Unknown)
    }

    fn build_node(tokens: Vec<Token>, value_hint: ExpectedResult) -> Result<EvalNode, DataError>
    {
        if let Some(root_index) = Self::index_of_root_operation(&tokens)
        {
            let tok = tokens[root_index].clone();
            if let Token::Operation(op) = tok
            {
                let num_op =  op.get_number_of_operands();
                if op.is_method()
                {
                    Self::parse_method(tokens, op, root_index)
                }
                else if num_op == 1
                {
                    Self::parse_unary(tokens, op, root_index)
                }
                else if num_op == 2
                {
                    Self::parse_binary(tokens, op, root_index)
                }
                else if op == Operation::Ternary
                {
                    Self::parse_ternary(tokens, op, root_index)
                }
                else
                {
                    return Err(DataError::Evaluation(EvalError::UnsupportedOperation))
                }
            }
            else
            {
                return Err(DataError::Tokenization(TokenizationError::OperationNotFound));
            }
        }
        else
        {
            // This is the base case of the recursive build, there should be
            // a operand node (and only one) in the list of expression tokens given.
            let mut result = None;
            for token in tokens
            {
                let found = match token
                {
                    Token::Tag(tag) =>
                            Some(match value_hint
                            {
                                ExpectedResult::Boolean => OperandNode::ReferencedCondition(tag),
                                ExpectedResult::Number => OperandNode::ReferencedValue(tag),
                                ExpectedResult::Unknown => OperandNode::ReferencedTag(tag),
                            }),
                    Token::OpenParen | Token::ClosedParen | Token::Comma | Token::Colon => None,
                    Token::Operation(o) => panic!("Found an operation token {:?} when expected none from not finding root index.", o),
                    Token::Number(n) => Some(OperandNode::ExplicitNumber(n)),
                    Token::Bool(b) => Some(OperandNode::ExplicitBool(b)),
                };

                if result.is_none() && found.is_some()
                {
                    result = found;
                }
                else if found.is_some()
                {
                    return Err(DataError::Tokenization(TokenizationError::MultipleOperandsFound));
                }
            }

            if let Some(result) = result
            {
                Ok(EvalNode::Operand(result))
            }
            else
            {
                Err(DataError::Tokenization(TokenizationError::OperandNotFound))
            }
        }
    }

    // Returns none if there is no operation found. This means there is only
    // an operand present.
    fn index_of_root_operation(tokens: &Vec<Token>) -> Option<usize>
    {
        let mut min_precedence = i32::MAX;
        let mut root_ind: Option<usize> = None;
        let mut brace_count = 0;

        let mut it = tokens.iter().enumerate();
        while let Some((i, t)) = it.next() {
            match t
            {
                Token::Bool(_) | Token::Number(_) | Token::Tag(_) | Token::Colon | Token::Comma => (),
                Token::OpenParen => brace_count = brace_count + 1,
                Token::ClosedParen => brace_count = brace_count - 1,
                Token::Operation(operation) => 
                {
                    let precedence = operation.get_precedence() + brace_count * 10;
                    if precedence < min_precedence
                    {
                        min_precedence = precedence;
                        root_ind = Some(i);
                    }
                },
            }
        }
        root_ind
    }

    fn parse_unary(tokens: Vec<Token>, op: Operation, root_index: usize) -> Result<EvalNode, DataError>
    {
        if tokens.iter().position(|t| !t.eq(&Token::OpenParen)).is_some_and(|i| i != root_index) || root_index != 0
        {
            return Err(DataError::SyntaxError(tokens.iter().nth(root_index).unwrap().clone()));
        }

        let child = match op.get_input_type(0)
        {
            Some(value_hint) => Self::build_node(parse::remove_parentheses(tokens[1..].to_vec()), value_hint)?,
            None => return Err(DataError::SyntaxError(tokens.iter().nth(root_index).unwrap().clone())),
        };
        Ok(EvalNode::Operation(OperationNode::new(op, vec![child])))
    }

    fn parse_binary(tokens: Vec<Token>, op: Operation, root_index: usize) -> Result<EvalNode, DataError>
    {
        if tokens.iter().position(|t| !t.eq(&Token::OpenParen)).is_some_and(|i| i == root_index) || root_index == 0
        {
            return Err(DataError::SyntaxError(tokens.iter().nth(root_index).unwrap().clone()))
        }

        let left = match op.get_input_type(0)
        {
            Some(value_hint) => Self::build_node(parse::remove_parentheses(tokens[..root_index].to_vec()), value_hint)?,
            None => return Err(DataError::SyntaxError(tokens.iter().nth(root_index).unwrap().clone())),
        };

        let right = match op.get_input_type(1)
        {
            Some(value_hint) => Self::build_node(parse::remove_parentheses(tokens[root_index + 1..].to_vec()), value_hint)?,
            None => return Err(DataError::SyntaxError(tokens.iter().nth(root_index).unwrap().clone())),
        };

        Ok(EvalNode::Operation(OperationNode::new(op, vec![left, right])))
    }

    fn parse_ternary(tokens: Vec<Token>, op: Operation, root_index: usize) -> Result<EvalNode, DataError>
    {
        let mut child_tokens = vec![];
        let mut num_paren = 0;
        for t in tokens.iter()
        {
            match t
            {
                Token::Comma | Token::Colon | Token::Tag(_) | Token::Number(_) | Token::Bool(_) => (),
                Token::OpenParen => num_paren += 1,
                Token::ClosedParen => num_paren -= 1,
                Token::Operation(o) =>
                {
                    if o.eq(&Operation::Ternary)
                    {
                        break;
                    }
                },
            }
        }

        child_tokens.push(Vec::from_iter(tokens[num_paren..root_index].iter().cloned()));

        num_paren = 0;

        for (i, t) in tokens.iter().enumerate()
        {
            match t
            {
                Token::OpenParen => (),
                Token::ClosedParen => num_paren += 1,
                Token::Colon => {
                    child_tokens.push(Vec::from_iter(tokens[root_index + 1..i].iter().cloned()));
                    child_tokens.push(Vec::from_iter(tokens[i + 1..tokens.len() - num_paren].iter().cloned()));
                    break;
                },
                Token::Comma | Token::Tag(_) | Token::Number(_) | Token::Bool(_) | Token::Operation(_) => (),
            }
        }
        
        if child_tokens.len() != 3
        {
            Err(DataError::SyntaxError(tokens.iter().nth(root_index).unwrap().clone()))
        }
        else
        {
            let mut children = vec![];
            
            for (i, v) in child_tokens.into_iter().enumerate()
            {
                children.push(Self::build_node(v, op.get_input_type(i).unwrap())?);
            }

            Ok(EvalNode::Operation(OperationNode::new(op, children)))
        }
    }

    fn parse_method(tokens: Vec<Token>, op: Operation, root_index: usize) -> Result<EvalNode, DataError>
    {
        if root_index + 1 >= tokens.len() || tokens.iter().nth(root_index + 1).unwrap().ne(&Token::OpenParen)
        {
            return Err(DataError::SyntaxError(tokens.iter().nth(root_index).unwrap().clone())) // The method call is empty
        }

        let mut child_tokens = vec![];
        let mut sub_tokens = vec![];

        let mut iter = tokens[root_index + 2..tokens.len() - 1].iter().peekable();
        while let Some(token) = iter.next()
        {
            sub_tokens.push(token.to_owned());
            if token.eq(&Token::Comma) || iter.peek().is_none()
            {
                child_tokens.push(sub_tokens.clone());
                sub_tokens = vec![];
            }
        }

        if child_tokens.len() != op.get_number_of_operands()
        {
            return Err(DataError::SyntaxError(tokens.iter().nth(root_index).unwrap().clone()))
        }

        let mut children = vec![];
        for (i, v) in child_tokens.into_iter().enumerate()
        {
            children.push(Self::build_node(remove_parentheses(v), op.get_input_type(i).unwrap())?);
        }
        Ok(EvalNode::Operation(OperationNode::new(op, children)))
    }

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
                OperandNode::ReferencedCondition(tag) =>
                            {
                                if ctx.has_conditional(tag)
                                {
                                    Ok(EvalResult::Boolean(ctx.eval_conditional(tag)?))
                                }
                                else
                                {
                                    Ok(EvalResult::Boolean(ctx.has_tag(tag)))
                                }
                            },
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
                                    Ok(EvalResult::Boolean(ctx.has_tag(tag)))
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

impl std::fmt::Display for EvalNode
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let s = match self
        {
            EvalNode::Operand(operand_node) =>
            match operand_node {
                OperandNode::ExplicitNumber(n) => format!("{}", n),
                OperandNode::ExplicitBool(b) => format!("{}", b),
                OperandNode::ReferencedValue(tag) | OperandNode::ReferencedCondition(tag) | OperandNode::ReferencedTag(tag) => format!("{}", tag.to_str()),
            },
            EvalNode::Operation(operation_node) => 
            match operation_node.get_operation()
            {
                Operation::Add => "+",
                Operation::Subtract => "-",
                Operation::Multiply => "*",
                Operation::Divide => "/",
                Operation::Negate => "-",
                Operation::PowSymbol => "^",
                Operation::PowMethod => "pow",
                Operation::Sqrt => "sqrt",
                Operation::Round => "round",
                Operation::RoundDown => "rounddown",
                Operation::RoundUp => "roundup",
                Operation::Range => "range",
                Operation::Ternary => "?",
                Operation::Equal => "==",
                Operation::NotEqual => "!=",
                Operation::LessThan => "<",
                Operation::LessThanEq => "<=",
                Operation::GreaterThan => ">",
                Operation::GreaterThanEq => ">=",
                Operation::Not => "!",
                Operation::Or => "||",
                Operation::And => "&&",
            }.to_string(),
        };
        write!(f, "{}", s)
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

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize, Clone)]
pub enum Operation
{
    // Expects numeric result
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
    PowSymbol,
    PowMethod,
    Sqrt,
    Round,
    RoundDown,
    RoundUp,
    Range,
    Ternary,
    // Expects boolean result
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

impl Operation
{
    fn get_number_of_operands(&self) -> usize
    {
        match self
        {
            Operation::Add | Operation::Subtract | Operation::Multiply | Operation::Divide | Operation::PowSymbol | Operation::PowMethod => 2,
            Operation::Negate | Operation::Sqrt => 1,
            Operation::Round | Operation::RoundDown | Operation::RoundUp => 1,
            Operation::Range | Operation::Ternary => 3,
            Operation::Equal | Operation::NotEqual | Operation::LessThan | Operation::LessThanEq | Operation::GreaterThan | Operation::GreaterThanEq => 2,
            Operation::Not => 1,
            Operation::Or | Operation::And => 2,
        }
    }

    fn is_method(&self) -> bool
    {
        match self
        {
            Operation::Add | Operation::Subtract | Operation::Multiply | Operation::Divide | Operation::Negate | Operation::PowSymbol => false,
            Operation::PowMethod | Operation::Sqrt | Operation::Round | Operation::RoundDown | Operation::RoundUp | Operation::Range => true,
            Operation::Ternary | Operation::Equal | Operation::NotEqual | Operation::LessThan | Operation::LessThanEq | Operation::GreaterThan | Operation::GreaterThanEq | Operation::Not | Operation::Or | Operation::And => false,
        }
    }

    fn get_precedence(&self) -> i32
    {
        match self
        {
            Operation::Add | Operation::Subtract => 1,
            Operation::Multiply | Operation::Divide => 2,
            Operation::Negate | Operation::PowSymbol | Operation::PowMethod => 3,
            Operation::Sqrt | Operation::Round | Operation::RoundDown | Operation::RoundUp | Operation::Range => 3,
            Operation::Ternary => 0,
            Operation::Equal | Operation::NotEqual | Operation::LessThan | Operation::LessThanEq | Operation::GreaterThan | Operation::GreaterThanEq => 2,
            Operation::Not | Operation::Or | Operation::And => 1,
        }
    }

    // Returns NONE if outside the range
    fn get_input_type(&self, input_index: usize) -> Option<ExpectedResult>
    {
        match self
        {
            Operation::Add | Operation::Subtract | Operation::Multiply | Operation::Divide | Operation::Negate | Operation::PowMethod | Operation::PowSymbol | Operation::Sqrt | Operation::Round | Operation::RoundDown | Operation::RoundUp | Operation::Range => 
                                if input_index < 2 
                                {
                                    Some(ExpectedResult::Number)
                                }
                                else 
                                {
                                    None
                                },
            Operation::Ternary => 
                                if input_index == 0
                                {
                                    Some(ExpectedResult::Boolean)
                                }
                                else if input_index == 1 || input_index == 2
                                {
                                    Some(ExpectedResult::Number)
                                }
                                else
                                {
                                    None
                                },
            Operation::Equal | Operation::NotEqual => Some(ExpectedResult::Unknown),
            Operation::LessThan | Operation::LessThanEq | Operation::GreaterThan | Operation::GreaterThanEq => 
                                if input_index < 2
                                {
                                    Some(ExpectedResult::Number)
                                }
                                else
                                {
                                    None
                                },
            Operation::Not => if input_index < 1
                                {
                                    Some(ExpectedResult::Boolean)
                                }
                                else
                                {
                                    None
                                },
            Operation::Or | Operation::And => if input_index < 2
                                {
                                    Some(ExpectedResult::Boolean)
                                }
                                else
                                {
                                    None
                                },
        }
    }
}

pub(super) mod parse
{
    use crate::api::data::{error::{EvalParseError, ParseError, ParseErrorType}, evaltree::{tokenize::Token, Operation}};

    /// Matching bracket implementation comes from StackOverflow:
    /// https://codereview.stackexchange.com/questions/253279/matching-brackets-in-rust
    enum Bracket {
        Open(char),
        Close(char),
    }

    impl Bracket {
        pub fn from_char(c: char) -> Option<Bracket> {
            match c {
                '{' | '[' | '(' => Some(Bracket::Open(c)),
                '}' => Some(Bracket::Close('{')),
                ']' => Some(Bracket::Close('[')),
                ')' => Some(Bracket::Close('(')),
                _ => None,
            }
        }
    }

    /// Check if the input `string` has balanced brackets.
    pub fn brackets_are_balanced(string: &str) -> Result<(), ParseError> {
        let mut brackets: Vec<char> = vec![];
        for (i, c) in string.chars().enumerate() {
            match Bracket::from_char(c) {
                Some(Bracket::Open(char_bracket)) => {
                    brackets.push(char_bracket);
                }
                Some(Bracket::Close(char_close_bracket)) => {
                    if brackets.pop() != Some(char_close_bracket) {
                        return Err(ParseError::new(string.to_string(), i, ParseErrorType::Evaluation(EvalParseError::MissingParentheses)));
                    }
                }
                _ => {}
            }
        }
        if brackets.is_empty()
        {
            Ok(())
        }
        else
        {
            Err(ParseError::new(string.to_string(), string.len(), ParseErrorType::Evaluation(EvalParseError::UnbalancedParentheses)))
        }
    }

    #[derive(Debug, Default, Clone)]
    struct ParenPair {
        left_pa: Option<usize>,
        min_op: Option<i32>,
        left_op: Option<i32>,
        is_method: bool,
    }

    impl ParenPair {
        fn new(left_pa: Option<usize>, is_method: bool) -> ParenPair {
            ParenPair {
                left_pa,
                is_method,
                min_op: None,
                left_op: None,
            }
        }
        fn new_empty() -> ParenPair {
            ParenPair {
                left_pa: None,
                min_op: None,
                left_op: None,
                is_method: false,
            }
        }
    }

    pub(super) fn remove_parentheses(tokens: Vec<Token>) -> Vec<Token>
    {
        let mut stack = vec![ParenPair::new_empty()];
        let mut unneeded = Vec::<usize>::new();
        let mut prev : Option<Operation> = None;

        for (i, token) in tokens.iter().enumerate()
        {
            let prev_is_method = prev.as_ref().is_some_and(|s| s.is_method());

            match token
            {
                Token::Comma | Token::Colon | Token::Number(_) | Token::Bool(_) | Token::Tag(_) => (),
                Token::OpenParen =>
                {
                    stack.push(ParenPair::new(Some(i), prev_is_method));
                },
                Token::ClosedParen => 
                {
                    if let Some(top) = stack.pop()
                    {
                        let mut needed = top.is_method;
                        if let Some(min_prec) = top.min_op
                        {
                            // Look to next right operation (if it exists)
                            let right: Option<&Operation> = tokens.iter().filter_map(|t| t.as_operation()).next();

                            // Check right precedence and keep parentheses if needed
                            if let Some(r_op) = right
                            {
                                if min_prec < r_op.get_precedence()
                                {
                                    needed = true;
                                }
                            }
                            else
                            {
                                if top.left_op.is_none() && stack.last().is_some()
                                {
                                    stack.iter_mut().last().map(|last| if !last.is_method { last.min_op = top.min_op });
                                }
                            }
                            // Check all previous left operations and keep parentheses if needed
                            for pair in stack.iter()
                            {
                                if let Some(left) = pair.left_op
                                {
                                    if left > min_prec
                                    {
                                        needed = true;
                                    }
                                }
                            }

                            if let Some(left) = top.left_op
                            {
                                if left > min_prec
                                {
                                    needed = true;
                                }
                            }
                        }

                        if !needed
                        {
                            if let Some(pa) = top.left_pa
                            {
                                unneeded.push(pa);
                                unneeded.push(i);
                            }
                        }
                    }
                    else
                    {
                        panic!(); // Unbalanced paren
                    }
                },
                Token::Operation(op) =>
                {
                    let default = &mut ParenPair::new_empty();
                    let paren_pair: &mut ParenPair = stack.last_mut().unwrap_or(default);

                    if paren_pair.min_op.is_none() || paren_pair.min_op.iter().any(|min| *min > op.get_precedence())
                    {
                        paren_pair.min_op = Some(op.get_precedence());
                    }

                    if !op.is_method()
                    {
                        paren_pair.left_op = Some(op.get_precedence());
                    }
                    prev = Some(op.clone());
                },
            }
            // Skip any operands
        }

        let mut result = vec![];
        for (i, s) in tokens.iter().enumerate() {
            if !unneeded.contains(&i) {
                result.push(s.clone());
            }
        }
        result
    }
}

pub mod tokenize
{
    use serde::{Deserialize, Serialize};

    use crate::api::data::{error::{EvalParseError, ParseError, ParseErrorType}, evaltree::Operation, tag::Tag};

    #[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
    pub enum Token
    {
        Tag(Tag),
        OpenParen,
        ClosedParen,
        Comma,
        Colon,
        // Placeholder(String),    // Defined using [placeholder_name], meant to be replaced with a tag
        Operation(Operation),
        Number(f32),
        Bool(bool),
    }

    impl Token
    {
        pub fn as_operation(&self) -> Option<&Operation>
        {
            if let Token::Operation(o) = self
            {
                Some(o)
            }
            else
            {
                None
            }
        }
    }

    /// Splits a given string into tokens
    /// so they can be more easily parsed into
    /// a ast.
    pub(super) fn tokenize_expression(s: &str) -> Result<Vec<Token>, ParseError>
    {
        let mut res = vec![];
        let s = remove_unneeded_whitespace(s);
        let chars: Vec<char> = s.chars().collect();
        let mut i = 0;

        while i < chars.len()
        {
            let c = chars[i];
            match c
            {
                '(' =>
                {
                    res.push(Token::OpenParen);
                    i += 1;
                },
                ')' =>
                {
                    res.push(Token::ClosedParen);
                    i += 1;
                },
                ',' =>
                {
                    res.push(Token::Comma);
                    i += 1;
                },
                '+' =>
                {
                    res.push(Token::Operation(Operation::Add));
                    i += 1;
                },
                '*' =>
                {
                    res.push(Token::Operation(Operation::Multiply));
                    i += 1;
                },
                '/' =>
                {
                    res.push(Token::Operation(Operation::Divide));
                    i += 1;
                },
                '^' =>
                {
                    res.push(Token::Operation(Operation::PowSymbol));
                    i += 1;
                },
                '?' =>
                {
                    res.push(Token::Operation(Operation::Ternary));
                    i += 1;
                },
                ':' => 
                {
                    res.push(Token::Colon);
                    i += 1;
                },
                '-' =>
                {
                    if let Some(l) = res.last()
                    {
                        let v = match l
                        {
                            Token::Tag(_) => Token::Operation(Operation::Subtract),
                            // Token::Placeholder(_) => Token::Operation(Operation::Subtract),
                            Token::OpenParen => Token::Operation(Operation::Negate),
                            Token::ClosedParen => Token::Operation(Operation::Subtract),
                            Token::Comma => Token::Operation(Operation::Negate),
                            Token::Colon => Token::Operation(Operation::Negate),
                            Token::Operation(_) => Token::Operation(Operation::Negate),
                            Token::Number(_) => Token::Operation(Operation::Negate),
                            Token::Bool(_) => return Err(ParseError::new(s.to_string(), i, ParseErrorType::Evaluation(EvalParseError::OperationTypeMismatch))),
                        };
                        res.push(v)
                    }
                    else
                    {
                        res.push(Token::Operation(Operation::Negate));
                    }
                    i += 1;
                },
                '=' =>
                {
                    if i + 1 < chars.len() && chars[i + 1] == '=' {
                        res.push(Token::Operation(Operation::Equal));
                        i += 2;
                    } else {
                        // Single = is not a valid token
                        return Err(ParseError::new(c.to_string(), i, ParseErrorType::Evaluation(EvalParseError::TokenInvalid)));
                    }
                },
                '!' =>
                {
                    if i + 1 < chars.len() && chars[i + 1] == '=' {
                        res.push(Token::Operation(Operation::NotEqual));
                        i += 2;
                    } else {
                        res.push(Token::Operation(Operation::Not));
                        i += 1;
                    }
                },
                '>' =>
                {
                    if i + 1 < chars.len() && chars[i + 1] == '=' {
                        res.push(Token::Operation(Operation::GreaterThanEq));
                        i += 2;
                    } else {
                        res.push(Token::Operation(Operation::GreaterThan));
                        i += 1;
                    }
                },
                '<' =>
                {
                    if i + 1 < chars.len() && chars[i + 1] == '=' {
                        res.push(Token::Operation(Operation::LessThanEq));
                        i += 2;
                    } else {
                        res.push(Token::Operation(Operation::LessThan));
                        i += 1;
                    }
                },
                '|' =>
                {
                    if i + 1 < chars.len() && chars[i + 1] == '|' {
                        res.push(Token::Operation(Operation::Or));
                        i += 2;
                    } else {
                        // Single = is not a valid token
                        return Err(ParseError::new(c.to_string(), i, ParseErrorType::Evaluation(EvalParseError::TokenInvalid)));
                    }
                },
                '&' =>
                {
                    if i + 1 < chars.len() && chars[i + 1] == '&' {
                        res.push(Token::Operation(Operation::And));
                        i += 2;
                    } else {
                        // Single = is not a valid token
                        return Err(ParseError::new(c.to_string(), i, ParseErrorType::Evaluation(EvalParseError::TokenInvalid)));
                    }
                },
                _ if c.is_digit(10) || (c == '.' && i + 1 < chars.len() && chars[i + 1].is_digit(10)) => 
                {
                    // Parse number
                    let mut j = i;
                    let mut has_dot = c == '.';
                    
                    while j < chars.len() && (chars[j].is_digit(10) || (chars[j] == '.' && !has_dot))
                    {
                        if chars[j] == '.'
                        {
                            if has_dot
                            {
                                return Err(ParseError::new(s.to_string(), j, ParseErrorType::Evaluation(EvalParseError::NumberMultipleDecimals)))
                            }
                            else
                            {
                                has_dot = true;
                            }
                        }
                        j += 1;
                    }
                    
                    let num_str = &s[i..j];

                    if let Ok(num) = num_str.parse()
                    {
                        res.push(Token::Number(num));
                    }
                    else
                    {
                        return Err(ParseError::new(s.to_string(), i, ParseErrorType::Evaluation(EvalParseError::TokenInvalid)));
                    }
                    i = j;
                },
                _ if c.is_alphabetic() =>
                {
                    // Parse identifier (could be bool, method, or tag)
                    let mut j = i;
                    
                    // Continue until we hit something that can't be part of an identifier
                    while j < chars.len()
                    {
                        let ch = chars[j];
                        if ch.is_alphanumeric() || ch == '.' || ch == ' '
                        {
                            // Check if space is part of a tag
                            if ch == ' ' {
                                // Look ahead to see if next non-space is '.' or alphanumeric
                                let mut k = j + 1;
                                while k < chars.len() && chars[k] == ' '
                                {
                                    k += 1;
                                }

                                if k < chars.len() && (chars[k].is_alphanumeric() || chars[k] == '.')
                                {
                                    j = k; // Skip to next non-space
                                }
                                else
                                {
                                    break; // Space is not part of identifier
                                }
                            }
                            else
                            {
                                j += 1;
                            }
                        }
                        else
                        {
                            break;
                        }
                    }
                    
                    let ident_str = &s[i..j];

                    let v;
                    if let Ok(b) = ident_str.parse()
                    {
                        v = Token::Bool(b);
                    }
                    else
                    {
                        v = match ident_str
                        {
                            "range" => Token::Operation(Operation::Range),
                            "round" => Token::Operation(Operation::Round),
                            "roundup" => Token::Operation(Operation::RoundUp),
                            "rounddown" => Token::Operation(Operation::RoundDown),
                            "pow" => Token::Operation(Operation::PowMethod),
                            "sqrt" => Token::Operation(Operation::Sqrt),
                            _ => 
                            {
                                if let Ok(tag) = Tag::from_str(ident_str)
                                {
                                    Token::Tag(tag)
                                }
                                else
                                {
                                    return Err(ParseError::new(s.to_string(), i, ParseErrorType::Evaluation(EvalParseError::TokenInvalid)))
                                }
                            },
                        }
                    }
                    res.push(v);
                    i = j;
                },
                _ =>
                {
                    // Unknown character
                    return Err(ParseError::new(c.to_string(), i, ParseErrorType::Evaluation(EvalParseError::TokenInvalid)));
                }
            }
        }

        Ok(res)
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
        use crate::api::data::{evaltree::{tokenize::{remove_unneeded_whitespace, tokenize_expression, Token}, Operation}, tag::Tag};

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

        /// Tests a simple expression calling a method (no syntax checks, just ensures tokenization)
        #[test]
        fn tokenize_test_1()
        {
            assert_eq!(tokenize_expression("rounddown()").unwrap(), vec![Token::Operation(Operation::RoundDown), Token::OpenParen, Token::ClosedParen]);
        }

        /// Tests a simple expression with a tag interior
        #[test]
        fn tokenize_test_2()
        {
            assert_eq!(tokenize_expression("rounddown( test . tag)").unwrap(), vec![Token::Operation(Operation::RoundDown), Token::OpenParen, Token::Tag(Tag::from_str("test.tag").unwrap()), Token::ClosedParen]);
        }

        /// Tests a large expression
        #[test]
        fn tokenize_test_3()
        {
            assert_eq!(tokenize_expression("rounddown((sqrt(8 * Ability.Magic Theory.Exp + 1)-1)/2)").unwrap(), vec![Token::Operation(Operation::RoundDown), Token::OpenParen, Token::OpenParen, Token::Operation(Operation::Sqrt), Token::OpenParen, Token::Number(8.0), Token::Operation(Operation::Multiply), Token::Tag(Tag::from_str("Ability.Magic Theory.Exp").unwrap()), Token::Operation(Operation::Add), Token::Number(1.0), Token::ClosedParen, Token::Operation(Operation::Subtract), Token::Number(1.0), Token::ClosedParen, Token::Operation(Operation::Divide), Token::Number(2.0), Token::ClosedParen]);
        }

        #[test]
        fn tokenize_test_4()
        {
            assert_eq!(tokenize_expression("Conditional.Tag == true").unwrap(), vec![Token::Tag(Tag::from_str("Conditional.Tag").unwrap()), Token::Operation(Operation::Equal), Token::Bool(true)]);
        }
    }
}

#[cfg(test)]
mod unit_tests
{
    use crate::api::data::{evaltree::EvalTree, tag::Tag, Context};

    #[test]
    fn equation_test_1()
    {
        let ctx = &Context::new();
        assert_eq!(EvalTree::from_str("rounddown(1.054)").unwrap().eval_as_num(ctx).unwrap(), 1.0);
    }

    #[test]
    fn equation_test_2()
    {
        let ctx = &mut Context::new();
        let tree = EvalTree::from_str("8 * 2").unwrap();
        println!("{}", tree);
        assert_eq!(tree.eval_as_num(ctx).unwrap(), 16.0);
    }

    #[test]
    fn equation_test_3()
    {
        let tree = EvalTree::from_str("rounddown((sqrt(8 * Ability.Magic Theory.Exp / 5 + 1)-1)/2)").unwrap();

        let ctx = &mut Context::new();
        ctx.set_attribute(&Tag::from_str("Ability.Magic Theory.Exp").unwrap(), 5.0).unwrap();
        assert_eq!(tree.eval_as_num(ctx).unwrap(), 1.0);

        ctx.set_attribute(&Tag::from_str("Ability.Magic Theory.Exp").unwrap(), 15.0).unwrap();
        assert_eq!(tree.eval_as_num(ctx).unwrap(), 2.0);
    }

    #[test]
    fn equation_test_4()
    {
        let tree = EvalTree::from_str("1.0 == Test").unwrap();

        let ctx = &mut Context::new();
        ctx.set_attribute(&Tag::from_str("Test").unwrap(), 1.0).unwrap();
        assert!(tree.eval_as_bool(ctx).unwrap());

        ctx.set_attribute(&Tag::from_str("Test").unwrap(), 3.0).unwrap();
        assert!(!tree.eval_as_bool(ctx).unwrap());
    }
}