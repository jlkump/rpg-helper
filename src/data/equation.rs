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

    pub fn evaluate<'a>(&'a self, expects: EvalResultType, container: &'a MetaTypeInstance, data: Option<&'a DataView>) -> Result<EvalResult, EvaluationError> {
        self.ast.evaluate(expects, container, data)
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

#[derive(Clone)]
pub enum EvalResult<'a, 'b> {
    Numeric(f32),
    MetaType(&'a MetaTypeInstance<'b>),
    Boolean(bool),
    Input(Vec<EvalRequest>, RequestEval),
    Request(EvalRequest)
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum EvalResultType {
    Any,
    Numeric,
    Boolean,
    MetaType,
}

impl<'a, 'b> EvalResult<'a, 'b> {
    pub fn as_bool(&self) -> Option<bool> {
        if let EvalResult::Boolean(b) = self {
            return Some(*b);
        }
        None
    }
    
    pub fn as_f32(&self, data: Option<&DataView>) -> Option<f32> {
        match &self {
            EvalResult::Numeric(n) => Some(*n),
            EvalResult::MetaType(m) => m.as_f32(data),
            _ => None
        }
    }

    pub fn as_meta_inst(&self) -> Option<&'a MetaTypeInstance<'b>> {
        if let EvalResult::MetaType(m) = self {
            return Some(m);
        }
        None
    }
    
    pub fn as_input(&self) -> Option<(Vec<EvalRequest>, RequestEval)> {
        match &self {
            EvalResult::Input(v, r) => Some((v.clone(), r.clone())),
            _ => None,
        }
    }

    pub fn as_request(&self) -> Option<EvalRequest> {
        match &self {
            EvalResult::Request(request) => Some(request.clone()),
            _ => None,
        }
    }

    fn process_input_or_request(&self, node: &mut OperatorNode, prev_requests: &mut Vec<EvalRequest>, val_requested: &mut bool) -> bool {
        // TODO: Need some way of not doing this repeatedly on the same input node requests
        //       That way we don't need to worry about calling expect repeatedly
        if *val_requested {
            return true;
        }

        if let Some((_, tree)) = self.as_input() {
            node.vals.push(tree.root);
            *val_requested = true;
            return true;
        } else if let Some(eval_request) = self.as_request() {
            node.vals.push(SyntaxNode::Input(prev_requests.len()));
            prev_requests.push(eval_request);
            *val_requested = true;
            return true;
        }
        false
    }

    fn expects_f32(&self, data: Option<&DataView<'_>>, node: &mut OperatorNode, prev_requests: &mut Vec<EvalRequest>, val_requested: &mut bool) -> Option<f32> {
        if self.process_input_or_request(node, prev_requests, val_requested) {
            None
        } else {
            self.as_f32(data)
        }
    }

    fn expects_inst(&self, node: &mut OperatorNode, prev_requests: &mut Vec<EvalRequest>, val_requested: &mut bool) -> Option<&'a MetaTypeInstance<'b>> {
        if self.process_input_or_request(node, prev_requests, val_requested) {
            None
        } else {
            self.as_meta_inst()
        }
    }

    fn expects_bool(&self, node: &mut OperatorNode, prev_requests: &mut Vec<EvalRequest>, val_requested: &mut bool) -> Option<bool> {
        todo!()
    }
}

#[derive(Clone, Debug)]
pub enum EvalRequest {
    DieRoll(DieRoll), // The requested die roll. 1d10, 2d3, Stress Die, etc
    Input(RestrictedInput) // Name of meta-type input
}

#[derive(Clone)]
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

    fn evaluate<'a>(&'a self, expects: EvalResultType, container: &'a MetaTypeInstance, data: Option<&'a DataView>) -> Result<EvalResult, EvaluationError> {
        self.root.eval_recursive(expects, container, data, None)
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
        SyntaxNode::Input(_) => (),
    }
}

#[derive(Debug, Clone, PartialEq)]
enum SyntaxNode {
    Operator(OperatorNode),
    Operand(OperandNode),
    Input(usize)
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
    fn eval_recursive<'a>(&'a self, expects: EvalResultType, container: &'a MetaTypeInstance, data: Option<&'a DataView>, inputs: Option<&'a Vec<Value>>) -> Result<EvalResult, EvaluationError> {
        match &self {
            SyntaxNode::Input(index) => {
                if let Some(inputs) = inputs {
                    if let Some(input) = inputs.iter().nth(*index) {
                        return Self::eval_input_node(expects, input, container, data);
                    }
                }
                return Err(EvaluationError);
            },
            SyntaxNode::Operator(op) => {
                // Perform operation if we can. If the eval result of children are requests, 
                // we need to build up the tree for the eval result to pass back up the chain. 
                
                
                // Operators will expect either a boolean, numeric or meta inst value and recieving 
                // the incorrect value will be an evaluation error.

                // First check that the expected value will match the value of the result for the operation
                if expects != EvalResultType::Any {
                    match op.op {
                        Operation::Add | Operation::Subtract | Operation::Multiply | 
                        Operation::Divide | Operation::Negate | Operation::Pow | Operation::Sqrt | 
                        Operation::Round | Operation::RoundDown | Operation::RoundUp => 
                            if expects != EvalResultType::Numeric {
                                return Err(EvaluationError);
                            },
                        Operation::Query => 
                            if expects != EvalResultType::MetaType || expects != EvalResultType::Numeric {
                                return Err(EvaluationError); 
                            },
                        Operation::Find => 
                            if expects != EvalResultType::MetaType {
                                return Err(EvaluationError);
                            },
                        Operation::Equal | Operation::NotEqual | Operation::LessThan | 
                        Operation::LessThanEq | Operation::GreaterThan | Operation::GreaterThanEq | 
                        Operation::Not | Operation::Or | Operation::And => 
                            if expects != EvalResultType::Boolean {
                                return Err(EvaluationError);
                            },
                        Operation::Ternary => {}, // Any type can be expected as a result
                    }
                }

                // Actually evaluate the node after type checks
                return Self::eval_operator_node(op, container, data, inputs);
            }, 
            SyntaxNode::Operand(o) => Self::eval_operand_node(expects, o, container, data), 
        }
    }

    fn eval_input_node<'a, 'b>(expects: EvalResultType, input: &'b Value<'a>, container: &'b MetaTypeInstance, data: Option<&'b DataView>) -> Result<EvalResult<'a, 'b>, EvaluationError> {
        match expects {
            EvalResultType::Numeric => if let Some(num) = input.as_f32(container, data) {
                return Ok(EvalResult::Numeric(num));
            },
            EvalResultType::Boolean => {},
            EvalResultType::MetaType => if let Some(inst) = input.as_meta_inst(data) {
                return Ok(EvalResult::MetaType(inst));
            },
            EvalResultType::Any => if let Some(inst) = input.as_meta_inst(data) {
                return Ok(EvalResult::MetaType(inst));
            } else if let Some(num) = input.as_f32(container, data) {
                return Ok(EvalResult::Numeric(num));
            },
        }
        return Err(EvaluationError);
    }

    fn eval_operator_node<'a, 'b>(op: &'a OperatorNode, container: &'a MetaTypeInstance, data: Option<&'a DataView<'b>>, inputs: Option<&'a Vec<Value>>) -> Result<EvalResult<'a, 'b>, EvaluationError> 
    where 'a: 'b
    {
        // Generally
        // - get the expected type
        // - if the type is instead an input or request type, build the resulting tree
        //      using the current operand
        let mut node = OperatorNode::new(op.op, vec![]);
        let mut prev_requests = vec![];
        let mut children_vals = vec![];
        if op.op != Operation::Find || op.op != Operation::Query {
            for (i, v) in op.vals[0..op.op.get_num_operands()].iter().enumerate() {
                children_vals.push(v.eval_recursive(op.op.expect_child_type(i), container, data, inputs)?);
            }
        }

        let mut v0_requested = false;
        let mut v1_requested = false;

        // Evaluate
        match op.op {
            Operation::Add => {
                if let Some(v0) = children_vals[0].expects_f32(data, &mut node, &mut prev_requests, &mut v0_requested) {
                    if let Some(v1) = children_vals[1].expects_f32(data, &mut node, &mut prev_requests, &mut v1_requested) {
                        return Ok(EvalResult::Numeric(v0 + v1));
                    }
                }
            },
            Operation::Subtract => {
                if let Some(v0) = children_vals[0].expects_f32(data, &mut node, &mut prev_requests, &mut v0_requested) {
                    if let Some(v1) = children_vals[1].expects_f32(data, &mut node, &mut prev_requests, &mut v1_requested) {
                        return Ok(EvalResult::Numeric(v0 - v1));
                    }
                }
            },
            Operation::Multiply => {
                if let Some(v0) = children_vals[0].expects_f32(data, &mut node, &mut prev_requests, &mut v0_requested) {
                    if let Some(v1) = children_vals[1].expects_f32(data, &mut node, &mut prev_requests, &mut v1_requested) {
                        return Ok(EvalResult::Numeric(v0 * v1));
                    }
                }
            },
            Operation::Divide => {
                if let Some(v0) = children_vals[0].expects_f32(data, &mut node, &mut prev_requests, &mut v0_requested) {
                    if let Some(v1) = children_vals[1].expects_f32(data, &mut node, &mut prev_requests, &mut v1_requested) {
                        return Ok(EvalResult::Numeric(v0 / v1));
                    }
                }
            },
            Operation::Negate => {
                if let Some(v0) = children_vals[0].expects_f32(data, &mut node, &mut prev_requests, &mut v0_requested) {
                    return Ok(EvalResult::Numeric(-v0));
                }
            },
            Operation::Pow => {
                if let Some(v0) = children_vals[0].expects_f32(data, &mut node, &mut prev_requests, &mut v0_requested) {
                    if let Some(v1) = children_vals[1].expects_f32(data, &mut node, &mut prev_requests, &mut v1_requested) {
                        return Ok(EvalResult::Numeric(v0.powf(v1)));
                    }
                }
            },
            Operation::Sqrt => {
                if let Some(v0) = children_vals[0].expects_f32(data, &mut node, &mut prev_requests, &mut v0_requested) {
                    return Ok(EvalResult::Numeric(v0.sqrt()));
                }
            },
            Operation::Round => {
                if let Some(v0) = children_vals[0].expects_f32(data, &mut node, &mut prev_requests, &mut v0_requested) {
                    return Ok(EvalResult::Numeric(v0.round()));
                }
            },
            Operation::RoundDown => {
                if let Some(v0) = children_vals[0].expects_f32(data, &mut node, &mut prev_requests, &mut v0_requested) {
                    return Ok(EvalResult::Numeric(v0.floor()));
                }
            },
            Operation::RoundUp => {
                if let Some(v0) = children_vals[0].expects_f32(data, &mut node, &mut prev_requests, &mut v0_requested) {
                    return Ok(EvalResult::Numeric(v0.ceil()));
                }
            },
            Operation::Ternary => todo!(),
            Operation::Query => {
                // Left operand should be an instance
                let eval = op.vals[0].eval_recursive(EvalResultType::MetaType, container, data, inputs)?;
                if let Some(inst) = eval.expects_inst(&mut node, &mut prev_requests, &mut v0_requested) {

                    if let SyntaxNode::Operator(node) = &op.vals[1] {
                        // Recursively eval right operand if it is an operator
                        return Self::eval_operator_node(node, inst, data, inputs);
                    } else if let SyntaxNode::Operand(node) = &op.vals[1] {
                        // If the right operand is a operand, it must be a query on the inst as a container
                        return Self::eval_operand_node(EvalResultType::Any, node, inst, None);
                    }
                }
            },
            Operation::Find => {
                if let Some(data) = data {

                    if let SyntaxNode::Operand(type_query) = &op.vals[0] {
                        if let OperandNode::Query(q) = type_query {
                            for t in data.get_all_of_type(q) {
                                // See which one matches first with op.vals[1] evaluated 
                                // to true as t being the input container
                                // Might need a container heirarchy being passes through the eval recursive,
                                // That way the "Super" qualifier can be used.
                                if let EvalResult::Boolean(b) = op.vals[1].eval_recursive(EvalResultType::Boolean, t.inst, Some(data), inputs)? {
                                    if b {
                                        return Ok(EvalResult::MetaType(&t.inst));
                                    }
                                }
                                
                            }
                        }
                    }
                }
                else 
                {
                    panic!()
                }
            },
            Operation::Equal => {
                // Could expect either a meta type or a numeric value
                if let Some(v0) = children_vals[0].expects_inst(&mut node, &mut prev_requests, &mut v0_requested) {
                    if let Some(v1) = children_vals[1].expects_inst(&mut node, &mut prev_requests, &mut v1_requested) {
                        // Currently compare for meta inst compares
                        //  First, if the types match and the data matches exactly or
                        //  Second, if the types both can be numbers, if their values are the same
                        return Ok(EvalResult::Boolean(v0.compare(v1, data)));
                    }
                } else if let Some(v0) = children_vals[0].expects_f32(data, &mut node, &mut prev_requests, &mut v0_requested) {
                    if let Some(v1) = children_vals[1].expects_f32(data, &mut node, &mut prev_requests, &mut v1_requested) {
                        return Ok(EvalResult::Boolean(v0 == v1));
                    }
                }
            },
            Operation::NotEqual => {
                if let Some(v0) = children_vals[0].expects_inst(&mut node, &mut prev_requests, &mut v0_requested) {
                    if let Some(v1) = children_vals[1].expects_inst(&mut node, &mut prev_requests, &mut v1_requested) {
                        // Currently compare for meta inst compares
                        //  First, if the types match and the data matches exactly or
                        //  Second, if the types both can be numbers, if their values are the same
                        return Ok(EvalResult::Boolean(!v0.compare(v1, data)));
                    }
                } else if let Some(v0) = children_vals[0].expects_f32(data, &mut node, &mut prev_requests, &mut v0_requested) {
                    if let Some(v1) = children_vals[1].expects_f32(data, &mut node, &mut prev_requests, &mut v1_requested) {
                        return Ok(EvalResult::Boolean(v0 != v1));
                    }
                }
            },
            Operation::LessThan => todo!(),
            Operation::LessThanEq => todo!(),
            Operation::GreaterThan => todo!(),
            Operation::GreaterThanEq => todo!(),
            Operation::Not => if let Some(v0) = children_vals[0].expects_bool(&mut node, &mut prev_requests, &mut v0_requested) {
                return Ok(EvalResult::Boolean(!v0));
            },
            Operation::Or => {
                if let Some(v0) = children_vals[0].expects_bool(&mut node, &mut prev_requests, &mut v0_requested) {
                    if v0 {
                        return Ok(EvalResult::Boolean(true));
                    }
                    if let Some(v1) = children_vals[1].expects_bool(&mut node, &mut prev_requests, &mut v1_requested) {
                        return Ok(EvalResult::Boolean(v0 || v1));
                    }
                }
            },
            Operation::And => {
                if let Some(v0) = children_vals[0].expects_bool(&mut node, &mut prev_requests, &mut v0_requested) {
                    if !v0 {
                        return Ok(EvalResult::Boolean(false));
                    }
                    if let Some(v1) = children_vals[1].expects_bool(&mut node, &mut prev_requests, &mut v1_requested) {
                        return Ok(EvalResult::Boolean(v0 && v1));
                    }
                }
            },
        }
        // TODO: If the first value was an input / request, then the second value has not been added to the 
        // list of requests and to the eval tree
        return Ok(EvalResult::Input(prev_requests, RequestEval {root: SyntaxNode::Operator(node)}));
    }



    fn eval_operand_node<'a, 'b>(expects: EvalResultType, o: &OperandNode, container: &'b MetaTypeInstance, data: Option<&'b DataView>) -> Result<EvalResult<'a, 'b>, EvaluationError> {
        // Result could be a number, eval of another meta type, a request for input or a die roll request.
        match o {
            OperandNode::Number(num) => 
                if expects == EvalResultType::Numeric {
                    return Ok(EvalResult::Numeric(*num))
                } else {
                    return Err(EvaluationError)
                },
            OperandNode::Boolean(b) => 
                if expects == EvalResultType::Boolean {
                    return Ok(EvalResult::Boolean(*b));
                } else {
                    return Err(EvaluationError)
                },
            OperandNode::Query(query) => {
                let mut v = None;

                if let Some(val) = container.get_field_value(query) {
                    // Check if container has the meta-type first
                    v = Some(val);
                } else if let Some(data) = data {
                    // Then check if the index has it
                    if let Some(val) = data.get_owned_index().get_values().get_value(query) {
                        v = Some(val);
                    }
                }
                
                if let Some(val) = v {
                    match expects {
                        EvalResultType::Numeric => {
                            if let Some(num) = val.as_f32(container, data) {
                                return Ok(EvalResult::Numeric(num));
                            }
                        },
                        EvalResultType::Boolean => {},
                        EvalResultType::MetaType => {
                            if let Some(meta_inst) = val.as_meta_inst(data) {
                                return Ok(EvalResult::MetaType(meta_inst));
                            }
                        },
                        EvalResultType::Any => {
                            if let Some(meta_inst) = val.as_meta_inst(data) {
                                return Ok(EvalResult::MetaType(meta_inst));
                            } else if let Some(num) = val.as_f32(container, data) {
                                return Ok(EvalResult::Numeric(num));
                            }
                        },
                    }

                    if let Some(input) = val.as_input() {
                        return Ok(EvalResult::Request(EvalRequest::Input(input.clone())));
                    } else if let Some(die_roll) = val.as_die_roll() {
                        return Ok(EvalResult::Request(EvalRequest::DieRoll(die_roll.clone())));
                    }
                }
                
                // When data is None, we expect the meta inst to have the field with the name
                return Err(EvaluationError);
            },
        }
    }

    fn add_to_input_index(&mut self, add: usize) {
        match self {
            SyntaxNode::Operator(op) => op.vals.iter_mut().for_each(|f| f.add_to_input_index(add)),
            SyntaxNode::Operand(_) => {},
            SyntaxNode::Input(i) => *i = *i + add,
        }
    }
}

impl Operation {
    fn expect_child_type(&self, child_index: usize) -> EvalResultType {
        match self {
            Operation::Add | Operation::Subtract | Operation::Multiply | Operation::Divide | 
            Operation::Negate | Operation::Pow | Operation::Sqrt | Operation::Round | 
            Operation::RoundDown | Operation::RoundUp => return EvalResultType::Numeric,
            Operation::Ternary => {
                if child_index == 0 {
                    return EvalResultType::Boolean;
                } else {
                    return EvalResultType::Any;
                }
            },
            Operation::Query => {
                todo!()
            },
            Operation::Find => {
                todo!()
            },
            Operation::Equal | Operation::NotEqual => EvalResultType::Any,
            Operation::LessThan | Operation::LessThanEq | 
            Operation::GreaterThan | Operation::GreaterThanEq => EvalResultType::Numeric, // Only can compare numbers with <, <=, >, >= (for now),
            Operation::Not | Operation::Or | Operation::And => EvalResultType::Boolean,
        }
    }
}

impl Display for SyntaxNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            SyntaxNode::Operator(node) => write!(f, "{}", node),
            SyntaxNode::Operand(node) => write!(f, "{}", node),
            SyntaxNode::Input(_) => todo!(),
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
    Boolean(bool),
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
            Self::Boolean(_) => todo!(),
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
    fn and_equal_field_query_parse() {
        let test= EquationSyntaxTree::build_syntax_tree("Spell::Technique == Technique && Spell::Form == Form".to_string()).unwrap().root;
        let expected = SyntaxNode::Operator(OperatorNode::new(Operation::And, 
            vec![
                SyntaxNode::Operator(OperatorNode::new(Operation::Equal, vec![
                    SyntaxNode::Operator(OperatorNode::new(Operation::Query, vec![
                        SyntaxNode::Operand(OperandNode::new("Spell".to_string()).unwrap()),
                        SyntaxNode::Operand(OperandNode::new("Technique".to_string()).unwrap())
                    ])), 
                    SyntaxNode::Operand(OperandNode::new("Technique".to_string()).unwrap())
                ])),
                SyntaxNode::Operator(OperatorNode::new(Operation::Equal, vec![
                    SyntaxNode::Operator(OperatorNode::new(Operation::Query, vec![
                        SyntaxNode::Operand(OperandNode::new("Spell".to_string()).unwrap()),
                        SyntaxNode::Operand(OperandNode::new("Form".to_string()).unwrap())
                    ])), 
                    SyntaxNode::Operand(OperandNode::new("Form".to_string()).unwrap())
                ]))
            ]));
        assert_eq!(test, expected);
    }

    #[test]
    fn and_mult_equal_field_query_parse() {
        let test= EquationSyntaxTree::build_syntax_tree("Spell::Technique == Technique && Spell::Form == Form && 1 != 2".to_string()).unwrap().root;
        let expected = SyntaxNode::Operator(OperatorNode::new(Operation::And, vec![
                SyntaxNode::Operator(OperatorNode::new(Operation::Equal, vec![
                    SyntaxNode::Operator(OperatorNode::new(Operation::Query, vec![
                        SyntaxNode::Operand(OperandNode::new("Spell".to_string()).unwrap()),
                        SyntaxNode::Operand(OperandNode::new("Technique".to_string()).unwrap())
                    ])), 
                    SyntaxNode::Operand(OperandNode::new("Technique".to_string()).unwrap())
                ])),
                SyntaxNode::Operator(OperatorNode::new(Operation::And, vec![
                    SyntaxNode::Operator(OperatorNode::new(Operation::Equal, vec![
                        SyntaxNode::Operator(OperatorNode::new(Operation::Query, vec![
                            SyntaxNode::Operand(OperandNode::new("Spell".to_string()).unwrap()),
                            SyntaxNode::Operand(OperandNode::new("Form".to_string()).unwrap())
                        ])), 
                        SyntaxNode::Operand(OperandNode::new("Form".to_string()).unwrap())
                    ])),
                    SyntaxNode::Operator(OperatorNode::new(Operation::NotEqual, vec![
                        SyntaxNode::Operand(OperandNode::new("1".to_string()).unwrap()),
                        SyntaxNode::Operand(OperandNode::new("2".to_string()).unwrap())
                    ]))
                ])),
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