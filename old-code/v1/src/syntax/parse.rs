use core::fmt;

use super::{operators::Operation, tokenize::tokenize_expression};

pub mod json_parser;

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
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets: Vec<char> = vec![];
    for c in string.chars() {
        match Bracket::from_char(c) {
            Some(Bracket::Open(char_bracket)) => {
                brackets.push(char_bracket);
            }
            Some(Bracket::Close(char_close_bracket)) => {
                if brackets.pop() != Some(char_close_bracket) {
                    return false;
                }
            }
            _ => {}
        }
    }
    brackets.is_empty()
}

fn string_contains_op(s: &str) -> bool {
    let tokens = tokenize_expression(s);
    tokens.len() != 1
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

#[derive(Debug, Clone)]
pub struct SyntaxError;

pub fn remove_paren(e: Vec<String>) -> Vec<String> {
    let mut stack = vec![ParenPair::new_empty()];
    let mut unneeded = Vec::<usize>::new();
    let mut prev : Option<&str> = None;
    for (i, token) in e.iter().enumerate() {
        let prev_is_method = prev.is_some_and(|s| Operation::is_method_operator(s));
        println!("On token {}, is op {:?}", token, Operation::get_operator(token, prev));
        if let Some(op) = Operation::get_operator(token, prev) {
            let default = &mut ParenPair::new_empty();
            let paren_pair: &mut ParenPair = stack.last_mut().unwrap_or(default);
            if paren_pair.min_op.is_none() || paren_pair.min_op.iter().any(|min| *min > op.get_precedence()) {
                paren_pair.min_op = Some(op.get_precedence());
            }
            if !Operation::is_method_operator(&token) {
                paren_pair.left_op = Some(op.get_precedence());
            }
        } else if token.eq("(") {
            stack.push(ParenPair::new(Some(i), prev_is_method));
        } else if token.eq(")") {
            println!("Stack is {:?}", stack);
            if let Some(top) = stack.pop() {
                let mut needed = top.is_method;
                if let Some(min_prec) = top.min_op {
                    let mut right: Option<Operation> = None;
                    // Look to next right operation (if it exists)
                    for n in (i + 1)..e.len() {
                        if e.iter().nth(n).is_some_and(|s| Operation::get_operator(s, e.iter().nth(n-1).map(|x| x.as_str())).is_some()) {
                            right = Operation::get_operator(e.iter().nth(n).unwrap(), e.iter().nth(n-1).map(|x| x.as_str()));
                            println!("Found right op for {} as {:?}", i, right);
                            break;
                        }
                        
                    }
                    // Check right precedence and keep parentheses if needed
                    if let Some(r_op) = right {
                        if min_prec < r_op.get_precedence() {
                            println!("Needed due to right precedence {} vs {}", min_prec, r_op.get_precedence());
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
                                println!("Needed due to left precedence {} vs {}", left, min_prec);
                                needed = true;
                            }
                        }
                    }
                    if let Some(left) = top.left_op {
                        if left > min_prec {
                            println!("Needed due to top left precedence {} vs {}", left, min_prec);
                            needed = true;
                        }
                    }
                }
                if !needed {
                    unneeded.push(top.left_pa.unwrap());
                    unneeded.push(i);
                }
            } else {
                panic!(); // Unbalanced paren
            }
        }
        prev = Some(token);
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

#[cfg(test)]
mod tests {
    use crate::syntax::{parse::{brackets_are_balanced, remove_paren}, tokenize::tokenize_expression};

    #[test]
    fn unbalanced() {
        assert!(!brackets_are_balanced("((Unbalanced)"))
    }

    #[test]
    fn remove_simple_paren() {
        let test = remove_paren(tokenize_expression("(4 + 2)"));
        let expected = tokenize_expression("4 + 2");
        assert_eq!(test, expected);
    }

    #[test]
    fn remove_simple_paren_two() {
        let test = remove_paren(tokenize_expression("2 * (4 + 2)"));
        let expected = tokenize_expression("2 * (4 + 2)");
        assert_eq!(test, expected);
    }

    #[test]
    fn remove_simple_paren_three() {
        let test = remove_paren(tokenize_expression("(4 + 2) * 2"));
        let expected = tokenize_expression("(4 + 2) * 2");
        assert_eq!(test, expected);
    }

    #[test]
    fn remove_simple_paren_four() {
        let test = remove_paren(tokenize_expression("((4 * 2) + 2)"));
        let expected = tokenize_expression("4 * 2 + 2");
        assert_eq!(test, expected);
    }

    #[test]
    fn remove_complex_paren() {
        let test = remove_paren(tokenize_expression("2 * (3) + ((4 * 2) + 2)"));
        let expected = tokenize_expression("2 * 3 + 4 * 2 + 2");
        assert_eq!(test, expected);
    }

    #[test]
    fn remove_complex_paren_two() {
        let test = remove_paren(tokenize_expression("2 * (3 * 3) + ((4 * 2) + 2)"));
        let expected = tokenize_expression("2 * 3 * 3 + 4 * 2 + 2");
        assert_eq!(test, expected);
    }

    #[test]
    fn remove_complex_paren_three() {
        let test = remove_paren(tokenize_expression("((sqrt(8 * Exp + 1)-1)/2)"));
        let expected = tokenize_expression("(sqrt(8 * Exp + 1)-1) / 2");
        assert_eq!(test, expected);
    }

    #[test]
    fn remove_ternary_paren() {
        let test = remove_paren(tokenize_expression("((1 < 2) ? (3 + 3) : 1) + 2"));
        let expected = tokenize_expression("(1 < 2 ? 3 + 3 : 1) + 2"); // Require paren around entire ternary
        assert_eq!(test, expected);
    }

    #[test]
    fn remove_ternary_paren_two() {
        let test = remove_paren(tokenize_expression("((1 < 2 || 3 > 2) ? (3 + 3) : 1) + 2"));
        let expected = tokenize_expression("((1 < 2 || 3 > 2) ? 3 + 3 : 1) + 2"); // Require paren around entire ternary, it should be find anyways
        assert_eq!(test, expected);
    }

    #[test]
    fn remove_ternary_paren_three() {
        let test = remove_paren(tokenize_expression("(1 < 2 || 3 > 2 ? (3 + 3) : 1) + 2"));
        let expected = tokenize_expression("(1 < 2 || 3 > 2 ? 3 + 3 : 1) + 2"); // Require paren around entire ternary, it should be find anyways
        assert_eq!(test, expected);
    }

    #[test]
    fn remove_ternary_paren_four() {
        let test = remove_paren(tokenize_expression("((Left::field) < Right::field ? Left::field : Right::field) + 2"));
        let expected = tokenize_expression("(Left::field < Right::field ? Left::field : Right::field) + 2");
        assert_eq!(test, expected);
    }

    #[test]
    fn remove_method_paren() {
        let test = remove_paren(tokenize_expression("sqrt(8)"));
        let expected = tokenize_expression("sqrt(8)");
        assert_eq!(test, expected);
    }

    #[test]
    fn remove_method_paren_two() {
        let test = remove_paren(tokenize_expression("sqrt((8))"));
        let expected = tokenize_expression("sqrt(8)");
        assert_eq!(test, expected);
    }

    #[test]
    fn remove_method_paren_three() {
        let test = remove_paren(tokenize_expression("sqrt((8 + 3))"));
        let expected = tokenize_expression("sqrt(8 + 3)");
        assert_eq!(test, expected);
    }

    #[test]
    fn remove_bool_paren() {
        let test = remove_paren(tokenize_expression("(1 < 2) || (1 > 2)"));
        let expected = tokenize_expression("1 < 2 || 1 > 2");
        assert_eq!(test, expected);
    }

    #[test]
    fn remove_bool_paren_two() {
        let test = remove_paren(tokenize_expression("(1 < 2 || 1 > 2) && (3 < 2 || 2 < 1)"));
        let expected = tokenize_expression("(1 < 2 || 1 > 2) && (3 < 2 || 2 < 1)");
        assert_eq!(test, expected);
    }

    #[test]
    fn compound_one() {
        let test = remove_paren(tokenize_expression("(3 < Test ? True : Not) + 3"));
        let expected = tokenize_expression("(3 < Test ? True : Not) + 3");
        assert_eq!(test, expected);
    }
}