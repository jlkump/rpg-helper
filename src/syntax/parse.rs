use core::fmt;


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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
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
    pub fn get_num_operands(&self) -> i32 {
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

    pub fn is_method_operator(s: &str) -> bool {
        match s {
            "sqrt" => true,
            "pow" => true,
            "round" => true,
            "rounddown" => true,
            "roundup" => true,
            _ => false,
        }
    }

    pub fn get_operator(s: &str, is_prefix: bool) -> Option<Operation> {
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

    pub fn get_precedence(&self) -> i32 {
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
    pub fn new(e: String, p: Option<usize>, er: ErrorType) -> SyntaxError {
        SyntaxError {
            equation_string: e,
            error_pos: p,
            error: er,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ErrorType {
    WrongNumOperands(Operation, i32, i32), // operation, has operands, should have
    MethodMissingParen(Operation), // operation
    EmptyParen,
    VariableInvalidChar,
    InvalidOperation,
    UnbalancedParen,
    UnknownChar(char)
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

pub fn remove_paren(e: Vec<String>) -> Vec<String> {
    let mut stack = vec![ParenPair::new_empty()];
    let mut unneeded = Vec::<usize>::new();
    let mut prev : Option<&String> = None;
    for (i, token) in e.iter().enumerate() {
        let is_between = prev.is_some_and(|s| s.starts_with(|c: char| c.is_alphanumeric() || c == ')'));
        let prev_is_method = prev.is_some_and(|s| Operation::is_method_operator(s));
        prev = Some(token);

        if let Some(op) = Operation::get_operator(token, !is_between) {
            let default = &mut ParenPair { min_op: None, left_op: None, left_pa: None, is_method: false};
            let paren_pair: &mut ParenPair = stack.last_mut().unwrap_or(default);
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
    fn remove_method_paren() {
        let test = remove_paren(tokenize_expression("sqrt(8)"));
        let expected = tokenize_expression("sqrt(8)");
        assert_eq!(test, expected);
    }
}