use core::fmt;
use std::{fmt::Display, num::ParseFloatError};

use super::dice::DieRoll;
use super::indexes::value_index::ValueIndex;
use super::meta_type::MetaTypeInstance;
use super::meta_type::RestrictedInput;
use super::meta_type::Type;
use super::meta_type::Value;
use super::DataView;
use crate::syntax::parse;
use crate::syntax::parse::remove_paren;
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

pub enum EvalResult<'a, 'b> {
    Numeric(f32),
    MetaType(&'a MetaTypeInstance<'b>),
    Boolean(bool),
    Request(Vec<EvalRequest>, RequestEval)
}

#[derive(Debug)]
pub enum EvalRequest {
    DieRoll(DieRoll), // The requested die roll. 1d10, 2d3, Stress Die, etc
    Input(RestrictedInput) // Name of meta-type input
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
            root: SyntaxNode::build_node(tokenize_expression(&e))?
        })
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
    fn build_node(e: Vec<String>) -> Result<SyntaxNode, SyntaxError> {
        println!("calling build root node for {:?}", e);
        if let Some(r) = Self::find_root_op_index(&e) {
            println!("Found root index {}", r);
            Self::parse_op(e, r)
        } else {
            println!("Did not find root index");
            // Trim paren and place the non-parentheses as
            // the leaf nodes
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
        if e[split].eq("?") {
            return Self::parse_ternary(e, split)
        } else {
            let prev;
            if split == 0 {
                prev = None;
            } else {
                prev = e.iter().nth(split - 1).map(|x| x.as_str());
            }
            if let Some(op) = Operation::get_operator(&e[split], prev) {
                if Operation::is_method_operator(&e[split]) {
                    return Self::parse_method_op(e, split, op)
                } else if op.get_num_operands() == 2 {
                    return Self::parse_binary_op(e, split, op)
                } else if op.get_num_operands() == 1 {
                    return Self::parse_unary_op(e, split, op)
                }
            }
        }
        Err(SyntaxError)
    }

    fn parse_method_op(e: Vec<String>, split: usize, op: Operation) -> Result<SyntaxNode, SyntaxError> {
        if split + 1 >= e.len() {
            return Err(SyntaxError) // The method call is empty
        }
        let mut params = vec![];
        let mut param = vec![];
        let iter = e[split + 2..].iter();
        let last = iter.len() - 1;
        for (i, token) in iter.enumerate() {
            if token.eq(",") || i == last {
                params.push(param.clone());
                param = vec![];
            } else {
                param.push(token.to_owned());
            }
        }
        if params.len() != op.get_num_operands() {
            return Err(SyntaxError)
        }
        let mut vals = vec![];
        for p in params {
            vals.push(Self::build_node(p)?)
        }
        Ok(SyntaxNode::Operator(OperatorNode::new(op, vals)))
    }

    fn parse_binary_op(e: Vec<String>, split: usize, op: Operation) -> Result<SyntaxNode, SyntaxError> {
        if e.iter().position(|s| !s.eq("(")).is_some_and(|i| i == split) || split == 0 {
            return Err(SyntaxError)
        }
        let left = remove_paren(Vec::from_iter(e[..split].iter().cloned()));
        let right = remove_paren(Vec::from_iter(e[split + 1..].iter().cloned()));
        return Ok(SyntaxNode::Operator(OperatorNode::new(op, vec![Self::build_node(left)?, Self::build_node(right)?])))
    }

    fn parse_unary_op(e: Vec<String>, split: usize, op: Operation) -> Result<SyntaxNode, SyntaxError> {
        if e.iter().position(|s| !s.eq("(")).is_some_and(|i| i != split) || split != 0 {
            return Err(SyntaxError)
        }
        let child = remove_paren(Vec::from_iter(e[split + 1..].iter().cloned()));
        return Ok(SyntaxNode::Operator(OperatorNode::new(op, vec![Self::build_node(child)?])))
    }
    
    fn parse_ternary(e: Vec<String>, split: usize) -> Result<SyntaxNode, SyntaxError> {
        let mut vals = vec![];
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
    }


    // TODO: Return Eval Request when equation requests input, such as a dice roll or selecting a reference to another value instance.
    // Optional data view so that we can force queries to be a value?
    fn eval_recursive(&self, container: &MetaTypeInstance, data: Option<&DataView>) -> Result<EvalResult, EvaluationError> {
        match &self {
            SyntaxNode::Operator(op) => {
                // Perform operation if we can. If the eval result of children are requests, 
                // we need to build up the tree for the eval result to pass back up the chain. 
                // Also, operands will expect either a boolean or a numeric value and recieving 
                // the incorrect value will be an evaluation error.

                // Expect a boolean result?
                // Expect a numeric result?

                // Evaluate
                match op.op {
                    Operation::Add => todo!(),
                    Operation::Subtract => todo!(),
                    Operation::Multiply => todo!(),
                    Operation::Divide => todo!(),
                    Operation::Negate => todo!(),
                    Operation::Pow => todo!(),
                    Operation::Sqrt => todo!(),
                    Operation::Round => todo!(),
                    Operation::RoundDown => todo!(),
                    Operation::RoundUp => todo!(),
                    Operation::Ternary => todo!(),
                    Operation::Query => {
                        // Look at the left operand node, query the 
                        todo!()
                    },
                    Operation::Find => {
                        if let SyntaxNode::Operand(type_query) = &op.vals[0] {
                            if let OperandNode::Query(q) = type_query {
                                // for t in data.get_value_index().all_of_type(q) {
                                //     // See which one matches first with op.vals[1] evaluated 
                                //     // to true as t being the input container
                                //     // Might need a container heirarchy being passes through the eval recursive,
                                //     // That way the "Super" qualifier can be used.
                                // }
                            }
                        }
                        todo!()
                    },
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
                todo!()
            }, 
            SyntaxNode::Operand(o) => {
                // Result could be a number, eval of another meta type, a request for input or a die roll request.

                // Is a number, return EvalResult of a numeric type
                match o {
                    OperandNode::Number(num) => {
                        return Ok(EvalResult::Numeric(*num))
                    },
                    OperandNode::Query(query) => {
                        // Check if container has the meta-type first

                        // Then check if the index has it
                        
                        // If neither has it, pass the query up the chain?

                        // If we found a value, check the type. If it is an input or die roll,
                        // pass a Request up the chain.
                    },
                }
                todo!()
            }, 
        }

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
                Err(SyntaxError)
            }
        }
    }
}

impl Display for OperandNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::Number(n) => write!(f, "{}", n),
            Self::Query(s) => write!(f, "{}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{data::equation::{Equation, EquationSyntaxTree, OperandNode, Operation, OperatorNode, SyntaxNode}, syntax::tokenize::tokenize_expression};

    #[test]
    fn simple_find_root() {
        assert_eq!(SyntaxNode::find_root_op_index(&vec![String::from("8"), String::from("+"), String::from("3")]), Some(1));
    }

    #[test]
    fn simple_find_no_root() {
        assert_eq!(SyntaxNode::find_root_op_index(&vec![String::from("83")]), None);
    }

    #[test]
    fn split_into_find_simple() {
        assert_eq!(SyntaxNode::find_root_op_index(&tokenize_expression("8 + 3")), Some(1));
    }

    #[test]
    fn split_into_find() {
        assert_eq!(SyntaxNode::find_root_op_index(&tokenize_expression("(8) + 3")), Some(3));
    }

    #[test]
    fn split_into_find_complex() {
        let split = tokenize_expression("rounddown((sqrt(8*Exp+1)-1)/2)");
        let i = SyntaxNode::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[0]);
        assert_eq!(i, 0);
    }

    #[test]
    fn split_into_find_complex_two() {
        let split = tokenize_expression("-rounddown((sqrt(8*Exp+1)-1)/2)");
        let i = SyntaxNode::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[0]);
        assert_eq!(i, 0);
    }

    #[test]
    fn split_into_find_complex_three() {
        let split = tokenize_expression("(2 * 3) + (4 * 2)");
        let i = SyntaxNode::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[5]);
        assert_eq!(i, 5);
    }

    #[test]
    fn split_into_find_complex_four() {
        let split = tokenize_expression("2 * 3 + 4 * 2");
        let i = SyntaxNode::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[3]);
        assert_eq!(i, 3);
    }

    #[test]
    fn split_into_find_complex_five() {
        let split = tokenize_expression("2 + 3 + 4 * 2");
        let i = SyntaxNode::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[1]);
        assert_eq!(i, 1);
    }

    #[test]
    fn split_into_find_complex_six() {
        let split = tokenize_expression("2 + 3 + 4 + 2 + 1");
        let i = SyntaxNode::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[1]);
        assert_eq!(i, 1);
    }

    #[test]
    fn split_into_find_complex_seven() {
        let split = tokenize_expression("(2) + (3) + 4 + 2 + 1");
        let i = SyntaxNode::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[3]);
        assert_eq!(i, 3);
    }

    #[test]
    fn split_into_find_complex_eight() {
        let split = tokenize_expression("rounddown(2) + (3) + 4 + 2 + 1");
        let i = SyntaxNode::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[4]);
        assert_eq!(i, 4);
    }

    #[test]
    fn split_into_find_complex_nine() {
        let split = tokenize_expression("-rounddown(2) + -(3) + 4 + 2 + 1");
        let i = SyntaxNode::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[5]);
        assert_eq!(i, 5);
    }

    #[test]
    fn split_into_find_complex_ten() {
        let split = tokenize_expression("2 + -(3) + 4 + 2 + 1");
        let i = SyntaxNode::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[1]);
        assert_eq!(i, 1);
    }

    #[test]
    fn split_into_find_complex_eleven() {
        let split = tokenize_expression("2^4 + -(3) + 4 + 2 + 1");
        let i = SyntaxNode::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[3]);
        assert_eq!(i, 3);
    }

    #[test]
    fn split_into_find_complex_twelve() {
        let split = tokenize_expression("1 + 2 + (4 + 2) + 1");
        let i = SyntaxNode::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[1]);
        assert_eq!(i, 1);
    }

    #[test]
    fn split_into_find_complex_thirteen() {
        let split = tokenize_expression("1 + 2 + (4 + 2) * 1");
        let i = SyntaxNode::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[1]);
        assert_eq!(i, 1);
    }

    #[test]
    fn bool_root() {
        let split = tokenize_expression("1 == 2");
        let i = SyntaxNode::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[1]);
        assert_eq!(i, 1);
    }

    #[test]
    fn ternary_root() {
        let split = tokenize_expression("1 == 2 ? 21 + 3 : 2");
        let i = SyntaxNode::find_root_op_index(&split).unwrap();
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
    fn simple_ternary_parse() {
        let test = SyntaxNode::parse_op(tokenize_expression("1 == 2 ? 21 : 3"), 3).unwrap();

        let expected = SyntaxNode::Operator(OperatorNode::new(Operation::Ternary, 
            vec![
                SyntaxNode::Operator(OperatorNode::new(Operation::Equal, vec![
                    SyntaxNode::Operand(OperandNode::new("1".to_string()).unwrap()),
                    SyntaxNode::Operand(OperandNode::new("2".to_string()).unwrap())
                ])), 
                SyntaxNode::Operand(OperandNode::new("21".to_string()).unwrap()), 
                SyntaxNode::Operand(OperandNode::new("3".to_string()).unwrap())
            ]));
        assert_eq!(test, expected);
    }

    #[test]
    fn simple_method_parse() {
        let test = EquationSyntaxTree::build_syntax_tree("find(Type, Field == Desired)".to_string()).unwrap().root;

        let expected = SyntaxNode::Operator(OperatorNode::new(Operation::Find, 
            vec![
                SyntaxNode::Operand(OperandNode::new("Type".to_string()).unwrap()),
                SyntaxNode::Operator(OperatorNode::new(Operation::Equal, vec![
                    SyntaxNode::Operand(OperandNode::new("Field".to_string()).unwrap()), 
                    SyntaxNode::Operand(OperandNode::new("Desired".to_string()).unwrap())
                ]))
            ]));
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