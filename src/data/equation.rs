use core::fmt;
use std::{fmt::Display, num::ParseFloatError};

use super::dice::DieRoll;
use super::indexes::value_index::ValueIndex;
use super::meta_type::MetaTypeInstance;
use super::meta_type::Type;
use super::meta_type::Value;
use super::DataView;
use crate::syntax::parse;
use crate::syntax::tokenize::tokenize_expression;
use crate::syntax::parse::SyntaxError;
use crate::syntax::operators::Operation;

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

    // When input is required for the equation, returns a list of the required input
    // that will be requested upon evaluation. Can be fed to a RequestEval
    // in order to evaluate the equation in full.
    pub fn get_required_input() -> Option<Vec<EvalRequest>> {
        todo!()
    }

    pub fn to_string(&self) -> String {
        // TODO: Reconstruct string from ast
        String::new()
    }
}

impl Display for Equation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug)]
pub struct EvaluationError;

pub enum EvalResult {
    Numeric(f32),
    Boolean(BoolEval),
    Request(Vec<EvalRequest>, RequestEval)
}

pub enum EvalRequest {
    DieRoll(DieRoll), // The requested die roll. 1d10, 2d3, Stress Die, etc
    Input(RestrictedValue) // Name of meta-type input
}

pub struct RestrictedValue {
    value_type: Type,
    restrictions: Vec<BoolEval>,
}

impl RestrictedValue {
    pub fn valid_input(given_input: &Value) -> bool {
        todo!()
    }
}

pub struct BoolEval {

}

impl BoolEval {
    pub fn evaluate<'a>(&self, input: &MetaTypeInstance<'a>, data: &DataView<'a>) -> bool {
        todo!()
    }
}

pub struct RequestEval {
    root: SyntaxNode // Built from the equation's AST, evaluated to the point where input is required. 
}

impl RequestEval {
    pub fn evaluate(inputs: Vec<Value>) -> f32 {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
struct EquationSyntaxTree {
    root: SyntaxNode
}

impl EquationSyntaxTree {
    fn build_syntax_tree(e: String) -> Result<EquationSyntaxTree, SyntaxError> {
        if !parse::brackets_are_balanced(&e) {
            return Err(SyntaxError)
        }
        Ok(EquationSyntaxTree {
            root: Self::build_node(tokenize_expression(&e))?
        })
    }

    fn build_node(e: Vec<String>) -> Result<SyntaxNode, SyntaxError> {
        if let Some(r) = Self::find_root_op_index(&e) {
            if let Ok(node) = Self::parse_op(e, r) {
                return Ok(node)
            } else {
                // Syntax error
                panic!()
            }
            // let left = parse::remove_paren(Vec::from_iter(e[0..r].iter().cloned()));
            // let right = parse::remove_paren(Vec::from_iter(e[r+1..].iter().cloned()));
            // let operator = &e[r];
            // let is_prefix_op = left.is_empty();
            // if let Some(op) = Operation::get_operator(operator, is_prefix_op) {
            //     let mut vals = vec![];
            //     if !left.is_empty() {
            //         vals.push(Self::build_node(left)?);
            //     }
            //     if !right.is_empty() {
            //         vals.push(Self::build_node(right)?);
            //     }
            //     return Ok(SyntaxNode::Operator(OperatorNode::new(op, vals)));
            // } else {
            //     return Err(SyntaxError::new(operator.to_owned(), None, ErrorType::InvalidOperation));
            // }
        } else {
            // Trim paren and place the non-parentheses as
            // the leaf nodes
            // let trimmed = remove_paren(e.to_vec());
            let operand = e.into_iter().find(|s| s.chars().all(|c: char| c.is_alphanumeric()));
            if operand.is_none() {
                return Err(SyntaxError);
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
        let mut prev = None;
        while let Some((i, s)) = it.next() {
            if s.eq(&"(") {
                brace_count = brace_count + 1;
            } else if s.eq(&")") {
                brace_count = brace_count - 1;
            } else {

                // let is_between = 
                //     i < e.len() - 1 && e.iter().nth(i + 1).unwrap().starts_with(|c: char| c.is_alphanumeric() || c == '(' || c == ')' || c == '-')
                //     && i > 0 && e.iter().nth(i - 1).unwrap().ends_with(|c: char| c.is_alphanumeric() || c == '(' || c == ')');
                if let Some(op) = Operation::get_operator(s, prev) {                    
                    let precedence = op.get_precedence() + brace_count * 10;
                    if precedence < min_precedence {
                        min_precedence = precedence;
                        root_ind = Some(i);
                    }
                }
            }
            prev = Some(s);
        }
        root_ind
    }

    fn parse_op(e: Vec<String>, split: usize) -> Result<SyntaxNode, SyntaxError> {
        let mut vals = vec![];
        match e[split].as_str() {
            "?" => {
                let mut num_paren = 0;
                for c in e.iter() {
                    if c.eq("(") {
                        num_paren += 1;
                    }
                    if c.eq(")") {
                        num_paren -= 1;
                    }
                    if c.eq("?") {
                        break;
                    }
                }
                vals.push(Vec::from_iter(e[num_paren..split].iter().cloned()));
                let expected = num_paren;
                num_paren = 0;
                for (i, c) in e.iter().enumerate() {
                    if c.eq(")") {
                        num_paren += 1;
                    }
                    if c.eq(":") && expected == num_paren {
                        vals.push(Vec::from_iter(e[split + 1..i].iter().cloned()));
                        vals.push(Vec::from_iter(e[i + 1..e.len() - num_paren].iter().cloned()));
                        break;
                    }
                }
                if vals.len() != 3 {
                    Err(SyntaxError)
                } else {
                    let mut children = vec![];
                    for v in vals {
                        children.push(Self::build_node(v)?);
                    }
                    Ok(SyntaxNode::Operator(OperatorNode::new(Operation::Ternary, children)))
                }
            },
            _ => {
                Err(SyntaxError)
            }
        }
        // match op {
        //     Operation::Add => todo!(),
        //     Operation::Subtract => todo!(),
        //     Operation::Multiply => todo!(),
        //     Operation::Divide => todo!(),
        //     Operation::Negate => todo!(),
        //     Operation::Pow => todo!(),
        //     Operation::Sqrt => todo!(),
        //     Operation::Round => todo!(),
        //     Operation::RoundDown => todo!(),
        //     Operation::RoundUp => todo!(),
        // }
    }

    fn evaluate(&self, container: &MetaTypeInstance, data: &ValueIndex) -> Result<f32, EvaluationError> {
        // self.root.eval_recursive(container, data)
        todo!()
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

impl SyntaxNode {
    // TODO: Return Eval Request when equation requests input, such as a dice roll or selecting a reference to another value instance.
    fn eval_recursive(&self, container: &MetaTypeInstance, data: &ValueIndex) -> Result<EvalResult, EvaluationError> {
        // match &self {
        //     SyntaxNode::Operand(op) => {
        //         match &op {
        //             OperandNode::Number(i) => Ok(EvalResult::Numeric(*i)),
        //             OperandNode::Query(q) => {
        //                 let mut val = None;
        //                 if let Some(fv) = container.get_field_value(q) {
        //                     if let Some(v) = fv.as_f32(container, data) {
        //                         val = Some(v);
        //                     }
        //                 }
        //                 if val.is_none() {
        //                     if let Some(i) = data.get_instance(q) {
        //                         if let Some(v) = i.get_field_value("Value") {
        //                             if let Some(v) = v.as_f32(container, data) {
        //                                 val = Some(v);
        //                             }
        //                         }
        //                     }
        //                 }
        //                 if let Some(v) = val {
        //                     Ok(v);
        //                 } else {
        //                     Err(EvaluationError); // Some Error
        //                 }
                        
        //             },
        //         }
        //     },
        // }
        //     SyntaxNode::Operator(op) => {
        //         match op.op {
        //             Operation::Add => Ok(Self::eval_recursive(&op.vals[0], container, data)? + Self::eval_recursive(&op.vals[1], container, data)?),
        //             Operation::Subtract => Ok(Self::eval_recursive(&op.vals[0], container, data)? - Self::eval_recursive(&op.vals[1], container, data)?),
        //             Operation::Multiply => Ok(Self::eval_recursive(&op.vals[0], container, data)? * Self::eval_recursive(&op.vals[1], container, data)?),
        //             Operation::Divide => Ok(Self::eval_recursive(&op.vals[0], container, data)? / Self::eval_recursive(&op.vals[1], container, data)?),
        //             Operation::Negate => Ok(-Self::eval_recursive(&op.vals[0], container, data)?),
        //             Operation::Pow => Ok(Self::eval_recursive(&op.vals[0], container, data)?.powf(Self::eval_recursive(&op.vals[1], container, data)?)),
        //             Operation::Sqrt => Ok(Self::eval_recursive(&op.vals[0], container, data)?.sqrt()),
        //             Operation::Round => Ok(Self::eval_recursive(&op.vals[0], container, data)?.round()),
        //             Operation::RoundDown => Ok(Self::eval_recursive(&op.vals[0], container, data)?.floor()),
        //             Operation::RoundUp => Ok(Self::eval_recursive(&op.vals[0], container, data)?.ceil()),
        //         }
        //     }
        // }
        todo!()
    }
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
            Operation::Ternary => todo!(),
            Operation::Query => todo!(),
            Operation::Find => todo!(),
            Operation::Equal => todo!(),
            Operation::NotEqual => todo!(),
            Operation::LessThan => todo!(),
            Operation::LessThanEq => todo!(),
            Operation::GreaterThan => todo!(),
            Operation::GreaterThanEq => todo!(),
            Operation::Not => todo!(),
            Operation::Or => todo!(),
            Operation::And => todo!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum OperandNode {
    Number(f32),
    Query(String),
    // InputRequest(RestrictedValue)    // Restricted Value is the input type and restrictions
    // DieRollRequest(DieRoll)          // DieRoll contains the type of die being rolled and how it is treated when rolled. 
    // IndexQuery(String, String, String), // IndexQuery(IndexID, type_name, field_name)
    // CompoundQuery(Vec<String>) // When the user uses the following syntax: Container::Field::Field of Field
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
                Err(SyntaxError)
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
    fn bool_root() {
        let split = tokenize_expression("1 == 2");
        let i = EquationSyntaxTree::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[1]);
        assert_eq!(i, 1);
    }

    #[test]
    fn ternary_root() {
        let split = tokenize_expression("1 == 2 ? 21 + 3 : 2");
        let i = EquationSyntaxTree::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[3]);
        assert_eq!(i, 3);
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