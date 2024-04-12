use core::fmt;
use std::{fmt::Display, num::ParseFloatError};

use super::ValueIndex;
use super::meta_type::MetaTypeInstance;
use crate::syntax::parse;
use crate::syntax::tokenize::tokenize_expression;
use crate::syntax::parse::SyntaxError;
use crate::syntax::parse::ErrorType;
use crate::syntax::parse::Operation;

#[derive(Debug, Clone, PartialEq)]
pub struct Equation {
    ast: EquationSyntaxTree
}

impl Equation {
    pub fn new(s: String) -> Result<Equation, SyntaxError> {
        Ok(Equation {
            ast: EquationSyntaxTree::build_syntax_tree(s)?
        })
    }

    pub fn evaluate(&self, container: &MetaTypeInstance, data: &ValueIndex) -> Result<f32, EvaluationError> {
        self.ast.evaluate(container, data)
    }
}

#[derive(Debug)]
pub struct EvaluationError;

#[derive(Debug, Clone, PartialEq)]
struct EquationSyntaxTree {
    root: SyntaxNode
}

impl EquationSyntaxTree {
    fn build_syntax_tree(e: String) -> Result<EquationSyntaxTree, SyntaxError> {
        if !parse::brackets_are_balanced(&e) {
            return Err(SyntaxError::new(e, None, ErrorType::UnbalancedParen))
        }
        Ok(EquationSyntaxTree {
            root: Self::build_node(tokenize_expression(&e))?
        })
    }

    fn build_node(e: Vec<String>) -> Result<SyntaxNode, SyntaxError> {
        if let Some(r) = Self::find_root_op_index(&e) {
            let left = parse::remove_paren(Vec::from_iter(e[0..r].iter().cloned()));
            let right = parse::remove_paren(Vec::from_iter(e[r+1..].iter().cloned()));
            let operator = &e[r];
            let is_prefix_op = left.is_empty();
            if let Some(op) = Operation::get_operator(operator, is_prefix_op) {
                let mut vals = vec![];
                if !left.is_empty() {
                    vals.push(Self::build_node(left)?);
                }
                if !right.is_empty() {
                    vals.push(Self::build_node(right)?);
                }
                return Ok(SyntaxNode::Operator(OperatorNode::new(op, vals)));
            } else {
                return Err(SyntaxError::new(operator.to_owned(), None, ErrorType::InvalidOperation));
            }
        } else {
            // Trim paren and place the non-parentheses as
            // the leaf nodes
            // let trimmed = remove_paren(e.to_vec());
            let operand = e.into_iter().find(|s| s.chars().all(|c: char| c.is_alphanumeric()));
            if operand.is_none() {
                return Err(SyntaxError::new("".to_owned(), None, ErrorType::EmptyParen));
            } else {
                return Ok(SyntaxNode::Operand(OperandNode::new(operand.unwrap())?));
            }
        }
    }

    /// Finds the place where the split needs to happen for the next syntax node
    /// If None is returned, we have hit a leaf node for the vec.
    fn find_root_op_index(e: &[String]) -> Option<usize> {
        let mut it = e.iter().enumerate();
        let mut min_precedence = i32::MAX;
        let mut root_ind: Option<usize> = None;
        let mut brace_count = 0;
        while let Some((i, s)) = it.next() {
            if s.eq(&"(") {
                brace_count = brace_count + 1;
            } else if s.eq(&")") {
                brace_count = brace_count - 1;
            } else {
                let is_between = 
                    i < e.len() - 1 && e.iter().nth(i + 1).unwrap().starts_with(|c: char| c.is_alphanumeric() || c == '(' || c == ')' || c == '-')
                    && i > 0 && e.iter().nth(i - 1).unwrap().ends_with(|c: char| c.is_alphanumeric() || c == '(' || c == ')');
                if let Some(op) = Operation::get_operator(s, !is_between) {                    
                    let precedence = op.get_precedence() + brace_count * 10;
                    if precedence < min_precedence {
                        min_precedence = precedence;
                        root_ind = Some(i);
                    }
                }
            }
            
        }
        root_ind
    }

    fn evaluate(&self, container: &MetaTypeInstance, data: &ValueIndex) -> Result<f32, EvaluationError> {
        Self::eval_recursive(&self.root, container, data)
    }

    fn eval_recursive(node: &SyntaxNode, container: &MetaTypeInstance, data: &ValueIndex) -> Result<f32, EvaluationError> {
        match &node {
            SyntaxNode::Operand(op) => {
                match &op {
                    OperandNode::Number(i) => Ok(*i),
                    OperandNode::Query(q) => {
                        if let Some(fv) = container.get_field_value(q) {
                            if let Some(val) = fv.as_f32(container, data) {
                                // TODO: Evaluate Enums as a query into the data?
                                // EX: Casting Score = Technique + Form + Stamina + Aura Modifier
                                Ok(val)
                            } else {
                                Err(EvaluationError) // Value is not a number, list, or reference to a number or list
                            }
                        } else if let Some(val) = data.get_value(q, "Value").as_f32(container, data) {
                            // If the container does not have the referenced value
                            Ok(val)
                        } else {
                            Err(EvaluationError) // Container or data does not have the requested field value
                        }
                    },
                }
            },
            SyntaxNode::Operator(op) => {
                match op.op {
                    Operation::Add => Ok(Self::eval_recursive(&op.vals[0], container, data)? + Self::eval_recursive(&op.vals[1], container, data)?),
                    Operation::Subtract => Ok(Self::eval_recursive(&op.vals[0], container, data)? - Self::eval_recursive(&op.vals[1], container, data)?),
                    Operation::Multiply => Ok(Self::eval_recursive(&op.vals[0], container, data)? * Self::eval_recursive(&op.vals[1], container, data)?),
                    Operation::Divide => Ok(Self::eval_recursive(&op.vals[0], container, data)? / Self::eval_recursive(&op.vals[1], container, data)?),
                    Operation::Negate => Ok(-Self::eval_recursive(&op.vals[0], container, data)?),
                    Operation::Pow => Ok(Self::eval_recursive(&op.vals[0], container, data)?.powf(Self::eval_recursive(&op.vals[1], container, data)?)),
                    Operation::Sqrt => Ok(Self::eval_recursive(&op.vals[0], container, data)?.sqrt()),
                    Operation::Round => Ok(Self::eval_recursive(&op.vals[0], container, data)?.round()),
                    Operation::RoundDown => Ok(Self::eval_recursive(&op.vals[0], container, data)?.floor()),
                    Operation::RoundUp => Ok(Self::eval_recursive(&op.vals[0], container, data)?.ceil()),
                }
            }
        }
    }
}

impl Display for EquationSyntaxTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        tree_recursive_display_helper(&mut result, &"".to_owned(), &self.root, true);
        write!(f, "{}", result)
    }
}

fn tree_recursive_display_helper(result: &mut String, prefix: &String, node: &SyntaxNode, end: bool) {
    result.push_str(prefix);
    result.push_str("|__");
    result.push_str(&node.to_string());
    result.push_str("\n");

    match node {
        SyntaxNode::Operator(node) => {
            let last = node.vals.len() - 1;
            for (i, n) in node.vals.iter().enumerate() {
                let mut new_prefix = prefix.clone();
                if end {
                    new_prefix.push_str("   ");
                } else {
                    new_prefix.push_str("|  ");
                }
                tree_recursive_display_helper(result, &new_prefix, n, i == last);
            }
        },
        SyntaxNode::Operand(_) => (),
    }
}

#[derive(Debug, Clone, PartialEq)]
enum SyntaxNode {
    Operator(OperatorNode),
    Operand(OperandNode),
}

impl Display for SyntaxNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            SyntaxNode::Operator(node) => write!(f, "{}", node),
            SyntaxNode::Operand(node) => write!(f, "{}", node),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct OperatorNode {
    op: Operation,
    vals: Vec<SyntaxNode>,
}

impl OperatorNode {
    fn new(op: Operation, vals: Vec<SyntaxNode>) -> OperatorNode {
        OperatorNode {
            op,
            vals
        }
    }
}

impl Display for OperatorNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.op {
            Operation::Add => write!(f, "+"),
            Operation::Subtract => write!(f, "-"),
            Operation::Multiply => write!(f, "*"),
            Operation::Divide => write!(f, "÷"),
            Operation::Negate => write!(f, "neg"),
            Operation::Pow => write!(f, "pow"),
            Operation::Sqrt => write!(f, "sqrt"),
            Operation::Round => write!(f, "round"),
            Operation::RoundDown => write!(f, "roundDown"),
            Operation::RoundUp =>  write!(f, "roundUp"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum OperandNode {
    Number(f32),
    Query(String)
}

impl OperandNode {
    fn new(s: String) -> Result<OperandNode, SyntaxError> {
        let num: Result<f32, ParseFloatError> = s.parse();
        if let Ok(n) = num {
            Ok(OperandNode::Number(n))
        } else {
            if s.chars().all(|c| c.is_alphabetic() || c == ' ') {
                Ok(OperandNode::Query(s))
            } else {
                // TODO: Determine if should be a number based error or a variable based error
                Err(SyntaxError::new(s, None, ErrorType::VariableInvalidChar))
            }
        }
    }
}

impl Display for OperandNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::Number(n) => write!(f, "{}", n),
            Self::Query(s) => write!(f, "{}", s)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{data::equation::{Equation, EquationSyntaxTree, OperandNode, Operation, OperatorNode, SyntaxNode}, syntax::tokenize::tokenize_expression};

    #[test]
    fn simple_find_root() {
        assert_eq!(EquationSyntaxTree::find_root_op_index(&vec![String::from("8"), String::from("+"), String::from("3")]), Some(1));
    }

    #[test]
    fn simple_find_no_root() {
        assert_eq!(EquationSyntaxTree::find_root_op_index(&vec![String::from("83")]), None);
    }

    #[test]
    fn split_into_find_simple() {
        assert_eq!(EquationSyntaxTree::find_root_op_index(&tokenize_expression("8 + 3")), Some(1));
    }

    #[test]
    fn split_into_find() {
        assert_eq!(EquationSyntaxTree::find_root_op_index(&tokenize_expression("(8) + 3")), Some(3));
    }

    #[test]
    fn split_into_find_complex() {
        let split = tokenize_expression("rounddown((sqrt(8*Exp+1)-1)/2)");
        let i = EquationSyntaxTree::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[0]);
        assert_eq!(i, 0);
    }

    #[test]
    fn split_into_find_complex_two() {
        let split = tokenize_expression("-rounddown((sqrt(8*Exp+1)-1)/2)");
        let i = EquationSyntaxTree::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[0]);
        assert_eq!(i, 0);
    }

    #[test]
    fn split_into_find_complex_three() {
        let split = tokenize_expression("(2 * 3) + (4 * 2)");
        let i = EquationSyntaxTree::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[5]);
        assert_eq!(i, 5);
    }

    #[test]
    fn split_into_find_complex_four() {
        let split = tokenize_expression("2 * 3 + 4 * 2");
        let i = EquationSyntaxTree::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[3]);
        assert_eq!(i, 3);
    }

    #[test]
    fn split_into_find_complex_five() {
        let split = tokenize_expression("2 + 3 + 4 * 2");
        let i = EquationSyntaxTree::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[1]);
        assert_eq!(i, 1);
    }

    #[test]
    fn split_into_find_complex_six() {
        let split = tokenize_expression("2 + 3 + 4 + 2 + 1");
        let i = EquationSyntaxTree::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[1]);
        assert_eq!(i, 1);
    }

    #[test]
    fn split_into_find_complex_seven() {
        let split = tokenize_expression("(2) + (3) + 4 + 2 + 1");
        let i = EquationSyntaxTree::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[3]);
        assert_eq!(i, 3);
    }

    #[test]
    fn split_into_find_complex_eight() {
        let split = tokenize_expression("rounddown(2) + (3) + 4 + 2 + 1");
        let i = EquationSyntaxTree::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[4]);
        assert_eq!(i, 4);
    }

    #[test]
    fn split_into_find_complex_nine() {
        let split = tokenize_expression("-rounddown(2) + -(3) + 4 + 2 + 1");
        let i = EquationSyntaxTree::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[5]);
        assert_eq!(i, 5);
    }

    #[test]
    fn split_into_find_complex_ten() {
        let split = tokenize_expression("2 + -(3) + 4 + 2 + 1");
        let i = EquationSyntaxTree::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[1]);
        assert_eq!(i, 1);
    }

    #[test]
    fn split_into_find_complex_eleven() {
        let split = tokenize_expression("2^4 + -(3) + 4 + 2 + 1");
        let i = EquationSyntaxTree::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[3]);
        assert_eq!(i, 3);
    }

    #[test]
    fn split_into_find_complex_twelve() {
        let split = tokenize_expression("1 + 2 + (4 + 2) + 1");
        let i = EquationSyntaxTree::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[1]);
        assert_eq!(i, 1);
    }

    #[test]
    fn split_into_find_complex_thirteen() {
        let split = tokenize_expression("1 + 2 + (4 + 2) * 1");
        let i = EquationSyntaxTree::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[1]);
        assert_eq!(i, 1);
    }

    #[test]
    fn simple_ast() {
        let test = Equation::new("sqrt(8)".to_owned()).unwrap().ast;
        let expected = EquationSyntaxTree {
            root: SyntaxNode::Operator(OperatorNode::new(Operation::Sqrt, vec![SyntaxNode::Operand(OperandNode::new("8".to_owned()).unwrap())]))
        };
        assert_eq!(test, expected);
    }

    #[test]
    fn arts_ast() {
        let test = Equation::new("rounddown((sqrt(8*Exp+1)-1)/2)".to_owned()).unwrap().ast;
        let expected = EquationSyntaxTree {
            root: SyntaxNode::Operator(OperatorNode::new(
                Operation::RoundDown, 
                vec![SyntaxNode::Operator(OperatorNode::new(
                    Operation::Divide,
                    vec![SyntaxNode::Operator(OperatorNode::new(
                            Operation::Subtract, 
                            vec![SyntaxNode::Operator(OperatorNode::new(
                                Operation::Sqrt,
                                vec![SyntaxNode::Operator(OperatorNode::new(
                                    Operation::Add,
                                    vec![SyntaxNode::Operator(OperatorNode::new(
                                        Operation::Multiply,
                                        vec![
                                            SyntaxNode::Operand(OperandNode::new("8".to_owned()).unwrap()),
                                            SyntaxNode::Operand(OperandNode::new("Exp".to_owned()).unwrap())
                                        ]
                                    )),
                                    SyntaxNode::Operand(OperandNode::new("1".to_owned()).unwrap())
                                    ]
                                ))]
                            )),
                                SyntaxNode::Operand(OperandNode::new("1".to_owned()).unwrap())
                            ]
                        )),
                        SyntaxNode::Operand(OperandNode::new("2".to_owned()).unwrap())
                    ]
                ))]
            ))
        };
        assert_eq!(test, expected);
    }
}