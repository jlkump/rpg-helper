
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

#[cfg(test)]
mod tests {
    use core::fmt;

    use crate::syntax::parse::{brackets_are_balanced};

    fn test_eq<T>(result: T, expected: T) where T: Eq + fmt::Debug + fmt::Display {
        assert_eq!(result, expected);

    }

    #[test]
    fn unbalanced() {
        assert!(!brackets_are_balanced("((Unbalanced)"))
    }
}