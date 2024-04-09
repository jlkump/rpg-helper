/// Does the "Lexer" portion of a Lexer and Parser for 
/// the contsturction of an AST
pub fn tokenize_expression(e: &str) -> Vec<String> {
    let e = remove_whitespace(e);
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

#[cfg(test)]
mod tests {
    use crate::syntax::tokenize::{remove_whitespace, tokenize_expression};

    #[test]
    fn simple_whitespace() {
        assert_eq!(remove_whitespace("rounddown( 5 )"), "rounddown(5)");
    }

    #[test]
    fn double_whitespace() {
        assert_eq!(remove_whitespace("rounddown(  5  )"), "rounddown(5)");
    }

    #[test]
    fn multiple_whitespace() {
        assert_eq!(remove_whitespace("rounddown     (     5     )"), "rounddown(5)");
    }

    #[test]
    fn variable_name_whitespace() {
        assert_eq!(remove_whitespace("rounddown(     5     + hello)"), "rounddown(5+hello)");
    }

    #[test]
    fn variable_name_contains_whitespace() {
        assert_eq!(remove_whitespace("rounddown(     5     + hello world)"), "rounddown(5+hello world)");
    }

    #[test]
    fn variable_name_contains_multiple_whitespace() {
        assert_eq!(remove_whitespace("rounddown(     5     + hello   world)"), "rounddown(5+hello   world)");
    }

    #[test]
    fn whitespace_only_variables() {
        assert_eq!(remove_whitespace("Technique + Form + Casting Score"), "Technique+Form+Casting Score");
    }

    #[test]
    fn whitespace_variables_and_methods() {
        assert_eq!(remove_whitespace("rounddown(Technique )+ Form  + sqrt(  Casting Score)"), "rounddown(Technique)+Form+sqrt(Casting Score)");
    }

    #[test]
    fn simple_method_split() {
        assert_eq!(tokenize_expression("rounddown(5)"), vec!["rounddown", "(", "5", ")"]);
    }

    #[test]
    fn simple_method_split_spaces() {
        assert_eq!(tokenize_expression("rounddown( 5 )"), vec!["rounddown", "(", "5", ")"]);
    }

    #[test]
    fn method_split() {
        assert_eq!(tokenize_expression("rounddown((sqrt(8 * Exp + 1)-1)/2)"), 
            vec!["rounddown", "(", "(", "sqrt", "(", "8", "*", "Exp", "+", "1", ")", "-", "1", ")", "/","2", ")"]);
    }

    #[test]
    fn method_split_excess_whitespace() {
        assert_eq!(tokenize_expression("rounddown  (    (    sqrt   (8   * Exp   +  1) -  1 )   / 2   )  "), 
            vec!["rounddown", "(", "(", "sqrt", "(", "8", "*", "Exp", "+", "1", ")", "-", "1", ")", "/","2", ")"]);
    }

    #[test]
    fn method_split_exotic_whitespace() {
        assert_eq!(tokenize_expression("rounddown  ( \t  \n (    sqrt   (8   *\t Exp   +  1) -  1 )   / 2   )  "), 
            vec!["rounddown", "(", "(", "sqrt", "(", "8", "*", "Exp", "+", "1", ")", "-", "1", ")", "/","2", ")"]);
    }

    #[test]
    fn negate_method() {
        assert_eq!(tokenize_expression("-rounddown  ( \t  \n (    sqrt   (8   *\t Exp   +  1) -  1 )   / 2   )  "), 
            vec!["-", "rounddown", "(", "(", "sqrt", "(", "8", "*", "Exp", "+", "1", ")", "-", "1", ")", "/","2", ")"]);
    }
}