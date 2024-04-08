use core::fmt;
use std::usize::MAX;

use super::{meta_type::MetaTypeInstance, CharacterData, DataIndex};
use crate::syntax::parse;

#[derive(Debug, Clone)]
pub struct Equation {
    inputs: Vec<String>,
    ast: EquationSyntaxTree
}

impl Equation {
    pub fn new() -> Equation {
        todo!()
    }

    pub fn get_inputs(&self) -> &Vec<String> {
        todo!()
    }

    pub fn evaluate(&self, container: &MetaTypeInstance, data: &DataIndex) -> i32 {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct EquationSyntaxTree {
    root: SyntaxNode
}

#[derive(Debug, Default, Clone)]
struct ParenPair {
    left_pa: Option<usize>,
    min_op: Option<i32>,
    left_op: Option<i32>,
    is_method: bool,
}

impl ParenPair {
    fn new(left_pa: Option<usize>, min_op: Option<i32>, left_op: Option<i32>, is_method: bool) -> ParenPair {
        ParenPair {
            left_pa,
            min_op,
            left_op,
            is_method,
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

impl EquationSyntaxTree {
    pub fn build_syntax_tree(e: String) -> Result<EquationSyntaxTree, SyntaxError> {
        if !parse::brackets_are_balanced(&e) {
            return Err(SyntaxError::new(e, None, ErrorType::UnbalancedParen))
        }
        todo!()
    }

    fn build_node(e: &mut [String]) -> SyntaxNode {
        if let Some(r) = Self::find_root_op_index(e) {

        } else {
            // Trim paren and place the non-parentheses as
            // the leaf nodes
        }
        todo!()
    }

    fn remove_paren(e: &[String]) -> Vec<String> {
        let mut stack = vec![ParenPair::new_empty()];
        let mut unneeded = Vec::<usize>::new();
        let mut prev : Option<&String> = None;
        for (i, token) in e.iter().enumerate() {
            let is_between = prev.is_some_and(|s| s.starts_with(|c: char| c.is_alphanumeric() || c == ')'));
            let prev_is_method = prev.is_some_and(|s| Operation::is_method_operator(s));
            prev = Some(token);

            if let Some(op) = Operation::get_operator(token, !is_between) {
                let paren_pair: &mut ParenPair = stack.last_mut().unwrap();
                if paren_pair.min_op.is_none() || paren_pair.min_op.iter().any(|min| *min > op.get_precedence()) {
                    paren_pair.min_op = Some(op.get_precedence());
                }
                if !Operation::is_method_operator(&token) {
                    paren_pair.left_op = Some(op.get_precedence());
                }
            } else if token.eq("(") {
                stack.push(ParenPair::new(Some(i), None, None, prev_is_method));
            } else if token.eq(")") {
                if let Some(top) = stack.pop() {
                    let mut needed = top.is_method;
                    if let Some(min_prec) = top.min_op {
                        let mut right: Option<Operation> = None;
                        // Look to next right operation (if it exists)
                        for n in (i + 1)..e.len() {
                            if e.iter().nth(n).is_some_and(|s| Operation::get_operator(s, false).is_some()) {
                                right = Operation::get_operator(e.iter().nth(n).unwrap(), false);
                            }
                        }
                        // Check right precedence and keep parentheses if needed
                        if let Some(r_op) = right {
                            if min_prec < r_op.get_precedence() {
                                needed = true;
                            }
                        } else {
                            if top.left_op.is_none() && stack.last().is_some() {
                                stack.iter_mut().last().map(|last| if !last.is_method { last.min_op = top.min_op });
                            }
                        }
                        // Check all previous left operations and keep parentheses if needed
                        for pair in stack.iter() {
                            if let Some(left) = pair.left_op {
                                if left > min_prec {
                                    needed = true;
                                }
                            }
                        }
                        // This often evaluates to true in cases when parentheses aren't needed (idk why the tutorial mentioned them)
                        // if let Some(left) = top.left_op {
                        //     if left == min_prec {
                        //         println!("Kept due to left precedence with {} and min {}", left, min_prec);
                        //         needed = true;
                        //     }
                        // }
                    }
                    if !needed {
                        println!("Pushing unneeded {}, {}", i, top.left_pa.unwrap());
                        unneeded.push(top.left_pa.unwrap());
                        unneeded.push(i);
                    }
                } else {
                    panic!(); // Unbalanced paren
                }
            }
            // Skip any operands
        }

        let mut result = vec![];
        for (i, s) in e.iter().enumerate() {
            if !unneeded.contains(&i) {
                result.push(s.clone());
            }
        }
        result
    }

    /// Finds the place where the split needs to happen for the next syntax node
    /// If None is returned, we have hit a leaf node for the vec.
    fn find_root_op_index(e: &[String]) -> Option<usize> {
        let mut it = e.iter().enumerate();
        let mut max_precedence = -1;
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
                    if precedence > max_precedence {
                        max_precedence = precedence;
                        root_ind = Some(i);
                    }
                }
            }
            
        }
        root_ind
    }

    /// Does the "Lexer" portion of a Lexer and Parser for 
    /// the contsturction of an AST
    fn tokenize_expression(e: &str) -> Vec<String> {
        let e = Self::remove_whitespace(e);
        let mut result = Vec::<String>::new();
        let mut last = 0;
        for (index, matched) in e.match_indices(|c: char| !c.is_alphanumeric() && c != ' ') {
            if last != index {
                result.push(e[last..index].to_string());
            }
            result.push(matched.to_string());
            last = index + matched.len();
        }
        if last < e.len() {
            result.push(e[last..].to_string());
        }
        result
    }

    /// Removes the unneccessary whitespace of an expression
    fn remove_whitespace(e: &str) -> String {
        let mut result = String::new();
        let mut previous: Option<char> = None;
        for (index, c) in e.chars().enumerate() {
            if c.is_whitespace() {
                if c == ' ' {
                    // Only include if the previous char was a alpha and next non-space is an alpha
                    if let Some(prev) = previous {
                        let mut i = index;
                        let mut next = e.chars().nth(i);
                        while next.is_some_and(|c| c == ' ') {
                            i = i + 1;
                            next = e.chars().nth(i);
                        }
                        if next.is_some_and(|next| next.is_alphabetic() && prev.is_alphabetic()) {
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
}

#[derive(Debug, Clone)]
enum SyntaxNode {
    Operator(OperatorNode),
    Operand(OperandNode),
}

#[derive(Debug, Clone)]
struct OperatorNode {
    op: Operation,
    vals: Vec<SyntaxNode>,
}

#[derive(Debug, Clone)]
enum OperandNode {
    Number(f32),
    Query(String)
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    RoundUp
}

impl Operation {
    fn get_num_operands(&self) -> i32 {
        match self {
            Operation::Add => 2,
            Operation::Subtract => 2,
            Operation::Multiply => 2,
            Operation::Divide => 2,
            Operation::Negate => 1,
            Operation::Pow => 2,
            Operation::Sqrt => 1,
            Operation::Round => 1,
            Operation::RoundUp => 1,
            Operation::RoundDown => 1,
        }
    }

    fn is_method_operator(s: &str) -> bool {
        match s {
            "sqrt" => true,
            "pow" => true,
            "round" => true,
            "rounddown" => true,
            "roundup" => true,
            _ => false,
        }
    }

    fn get_operator(s: &str, is_prefix: bool) -> Option<Operation> {
        match s {
            "sqrt" => Some(Operation::Sqrt),
            "pow" => Some(Operation::Pow),
            "round" => Some(Operation::Round),
            "rounddown" => Some(Operation::RoundDown),
            "roundup" => Some(Operation::RoundUp),
            "+" => if is_prefix {
                    None
                } else {
                    Some(Operation::Add)
                },
            "-" => if is_prefix {
                    Some(Operation::Negate)
                } else {
                    Some(Operation::Subtract)
                },
            "*" => if is_prefix {
                    None
                } else {
                    Some(Operation::Multiply)
                },
            "/" => if is_prefix {
                    None
                } else {
                    Some(Operation::Divide)
                },
            "^" => if is_prefix {
                    None
                } else {
                    Some(Operation::Pow)
                },
            _ => None
        }
    }

    fn get_precedence(&self) -> i32 {
        match &self {
            Operation::Add => 0,
            Operation::Subtract => 0,
            Operation::Multiply => 1,
            Operation::Divide => 1,
            Operation::Negate => 2,
            Operation::Pow => 2,
            Operation::Sqrt => 2,
            Operation::Round => 2,
            Operation::RoundUp => 2,
            Operation::RoundDown => 2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SyntaxError {
    equation_string: String,
    error_pos: Option<usize>,
    error: ErrorType
}

impl SyntaxError {
    fn new(e: String, p: Option<usize>, er: ErrorType) -> SyntaxError {
        SyntaxError {
            equation_string: e,
            error_pos: p,
            error: er,
        }
    }
}

#[derive(Debug, Clone)]
enum ErrorType {
    WrongNumOperands(Operation, i32, i32), // operation, has operands, should have
    MethodMissingParen(Operation), // operation
    UnbalancedParen,
    UnknownChar(char)
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::data::equation::{EquationSyntaxTree, Operation};

    #[test]
    fn simple_whitespace() {
        assert_eq!(EquationSyntaxTree::remove_whitespace("rounddown( 5 )"), "rounddown(5)");
    }

    #[test]
    fn double_whitespace() {
        assert_eq!(EquationSyntaxTree::remove_whitespace("rounddown(  5  )"), "rounddown(5)");
    }

    #[test]
    fn multiple_whitespace() {
        assert_eq!(EquationSyntaxTree::remove_whitespace("rounddown     (     5     )"), "rounddown(5)");
    }

    #[test]
    fn variable_name_whitespace() {
        assert_eq!(EquationSyntaxTree::remove_whitespace("rounddown(     5     + hello)"), "rounddown(5+hello)");
    }

    #[test]
    fn variable_name_contains_whitespace() {
        assert_eq!(EquationSyntaxTree::remove_whitespace("rounddown(     5     + hello world)"), "rounddown(5+hello world)");
    }

    #[test]
    fn variable_name_contains_multiple_whitespace() {
        assert_eq!(EquationSyntaxTree::remove_whitespace("rounddown(     5     + hello   world)"), "rounddown(5+hello   world)");
    }

    #[test]
    fn whitespace_only_variables() {
        assert_eq!(EquationSyntaxTree::remove_whitespace("Technique + Form + Casting Score"), "Technique+Form+Casting Score");
    }

    #[test]
    fn whitespace_variables_and_methods() {
        assert_eq!(EquationSyntaxTree::remove_whitespace("rounddown(Technique )+ Form  + sqrt(  Casting Score)"), "rounddown(Technique)+Form+sqrt(Casting Score)");
    }

    #[test]
    fn simple_method_split() {
        assert_eq!(EquationSyntaxTree::tokenize_expression("rounddown(5)"), vec!["rounddown", "(", "5", ")"]);
    }

    #[test]
    fn simple_method_split_spaces() {
        assert_eq!(EquationSyntaxTree::tokenize_expression("rounddown( 5 )"), vec!["rounddown", "(", "5", ")"]);
    }

    #[test]
    fn method_split() {
        assert_eq!(EquationSyntaxTree::tokenize_expression("rounddown((sqrt(8 * Exp + 1)-1)/2)"), 
            vec!["rounddown", "(", "(", "sqrt", "(", "8", "*", "Exp", "+", "1", ")", "-", "1", ")", "/","2", ")"]);
    }

    #[test]
    fn method_split_excess_whitespace() {
        assert_eq!(EquationSyntaxTree::tokenize_expression("rounddown  (    (    sqrt   (8   * Exp   +  1) -  1 )   / 2   )  "), 
            vec!["rounddown", "(", "(", "sqrt", "(", "8", "*", "Exp", "+", "1", ")", "-", "1", ")", "/","2", ")"]);
    }

    #[test]
    fn method_split_exotic_whitespace() {
        assert_eq!(EquationSyntaxTree::tokenize_expression("rounddown  ( \t  \n (    sqrt   (8   *\t Exp   +  1) -  1 )   / 2   )  "), 
            vec!["rounddown", "(", "(", "sqrt", "(", "8", "*", "Exp", "+", "1", ")", "-", "1", ")", "/","2", ")"]);
    }

    #[test]
    fn negate_method() {
        assert_eq!(EquationSyntaxTree::tokenize_expression("-rounddown  ( \t  \n (    sqrt   (8   *\t Exp   +  1) -  1 )   / 2   )  "), 
            vec!["-", "rounddown", "(", "(", "sqrt", "(", "8", "*", "Exp", "+", "1", ")", "-", "1", ")", "/","2", ")"]);
    }

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
        assert_eq!(EquationSyntaxTree::find_root_op_index(&EquationSyntaxTree::tokenize_expression("8 + 3")), Some(1));
    }

    #[test]
    fn split_into_find() {
        assert_eq!(EquationSyntaxTree::find_root_op_index(&EquationSyntaxTree::tokenize_expression("(8) + 3")), Some(3));
    }

    #[test]
    fn split_into_find_complex() {
        let split = EquationSyntaxTree::tokenize_expression("rounddown((sqrt(8*Exp+1)-1)/2)");
        let i = EquationSyntaxTree::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[6]);
        assert_eq!(i, 6);
    }

    #[test]
    fn split_into_find_complex_two() {
        let split = EquationSyntaxTree::tokenize_expression("-rounddown((sqrt(8*Exp+1)-1)/2)");
        let i = EquationSyntaxTree::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[7]);
        assert_eq!(i, 7);
    }

    #[test]
    fn split_into_find_complex_three() {
        let split = EquationSyntaxTree::tokenize_expression("(2 * 3) + (4 * 2)");
        let i = EquationSyntaxTree::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[2]);
        assert_eq!(i, 2);
    }

    #[test]
    fn split_into_find_complex_four() {
        let split = EquationSyntaxTree::tokenize_expression("2 * 3 + 4 * 2");
        let i = EquationSyntaxTree::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[1]);
        assert_eq!(i, 1);
    }

    #[test]
    fn split_into_find_complex_five() {
        let split = EquationSyntaxTree::tokenize_expression("2 + 3 + 4 * 2");
        let i = EquationSyntaxTree::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[5]);
        assert_eq!(i, 5);
    }

    #[test]
    fn split_into_find_complex_six() {
        let split = EquationSyntaxTree::tokenize_expression("2 + 3 + 4 + 2 + 1");
        let i = EquationSyntaxTree::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[1]);
        assert_eq!(i, 1);
    }

    #[test]
    fn split_into_find_complex_seven() {
        let split = EquationSyntaxTree::tokenize_expression("(2) + (3) + 4 + 2 + 1");
        let i = EquationSyntaxTree::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[3]);
        assert_eq!(i, 3);
    }

    #[test]
    fn split_into_find_complex_eight() {
        let split = EquationSyntaxTree::tokenize_expression("rounddown(2) + (3) + 4 + 2 + 1");
        let i = EquationSyntaxTree::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[0]);
        assert_eq!(i, 0);
    }

    #[test]
    fn split_into_find_complex_nine() {
        let split = EquationSyntaxTree::tokenize_expression("-rounddown(2) + -(3) + 4 + 2 + 1");
        let i = EquationSyntaxTree::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[0]);
        assert_eq!(i, 0);
    }

    #[test]
    fn split_into_find_complex_ten() {
        let split = EquationSyntaxTree::tokenize_expression("2 + -(3) + 4 + 2 + 1");
        let i = EquationSyntaxTree::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[2]);
        assert_eq!(i, 2);
    }

    #[test]
    fn split_into_find_complex_eleven() {
        let split = EquationSyntaxTree::tokenize_expression("2^4 + -(3) + 4 + 2 + 1");
        let i = EquationSyntaxTree::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[1]);
        assert_eq!(i, 1);
    }

    #[test]
    fn split_into_find_complex_twelve() {
        let split = EquationSyntaxTree::tokenize_expression("1 + 2 + (4 + 2) + 1");
        let i = EquationSyntaxTree::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[6]);
        assert_eq!(i, 6);
    }

    #[test]
    fn split_into_find_complex_thirteen() {
        let split = EquationSyntaxTree::tokenize_expression("1 + 2 + (4 + 2) * 1");
        let i = EquationSyntaxTree::find_root_op_index(&split).unwrap();
        assert_eq!(split[i], split[6]);
        assert_eq!(i, 6);
    }

    #[test]
    fn remove_simple_paren() {
        let test = EquationSyntaxTree::remove_paren(&EquationSyntaxTree::tokenize_expression("(4 + 2)"));
        let expected = EquationSyntaxTree::tokenize_expression("4 + 2");
        assert_eq!(test, expected);
    }

    #[test]
    fn remove_simple_paren_two() {
        let test = EquationSyntaxTree::remove_paren(&EquationSyntaxTree::tokenize_expression("2 * (4 + 2)"));
        let expected = EquationSyntaxTree::tokenize_expression("2 * (4 + 2)");
        assert_eq!(test, expected);
    }

    #[test]
    fn remove_simple_paren_three() {
        let test = EquationSyntaxTree::remove_paren(&EquationSyntaxTree::tokenize_expression("(4 + 2) * 2"));
        let expected = EquationSyntaxTree::tokenize_expression("(4 + 2) * 2");
        assert_eq!(test, expected);
    }

    #[test]
    fn remove_simple_paren_four() {
        let test = EquationSyntaxTree::remove_paren(&EquationSyntaxTree::tokenize_expression("((4 * 2) + 2)"));
        let expected = EquationSyntaxTree::tokenize_expression("4 * 2 + 2");
        assert_eq!(test, expected);
    }

    #[test]
    fn remove_complex_paren() {
        let test = EquationSyntaxTree::remove_paren(&EquationSyntaxTree::tokenize_expression("2 * (3) + ((4 * 2) + 2)"));
        let expected = EquationSyntaxTree::tokenize_expression("2 * 3 + 4 * 2 + 2");
        assert_eq!(test, expected);
    }

    #[test]
    fn remove_complex_paren_two() {
        let test = EquationSyntaxTree::remove_paren(&EquationSyntaxTree::tokenize_expression("2 * (3 * 3) + ((4 * 2) + 2)"));
        let expected = EquationSyntaxTree::tokenize_expression("2 * 3 * 3 + 4 * 2 + 2");
        assert_eq!(test, expected);
    }

    #[test]
    fn remove_complex_paren_three() {
        let test = EquationSyntaxTree::remove_paren(&EquationSyntaxTree::tokenize_expression("((sqrt(8 * Exp + 1)-1)/2)"));
        let expected = EquationSyntaxTree::tokenize_expression("(sqrt(8 * Exp + 1)-1) / 2");
        assert_eq!(test, expected);
    }

    #[test]
    fn remove_method_paren() {
        let test = EquationSyntaxTree::remove_paren(&EquationSyntaxTree::tokenize_expression("sqrt(8)"));
        let expected = EquationSyntaxTree::tokenize_expression("sqrt(8)");
        assert_eq!(test, expected);
    }
}