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
    RoundUp,
    Ternary,
    Query,
    Find,
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
            Operation::Ternary => 3,
            Operation::Query => 2,
            Operation::Find => 2,
            Operation::Equal => 2,
            Operation::NotEqual => 2,
            Operation::LessThan => 2,
            Operation::LessThanEq => 2,
            Operation::GreaterThan => 2,
            Operation::GreaterThanEq => 2,
            Operation::Not => 1,
            Operation::Or => 2,
            Operation::And => 2,
        }
    }

    pub fn is_method_operator(s: &str) -> bool {
        match s {
            "sqrt" | "pow" | "round" | "rounddown" | "roundup" | "find" => true,
            _ => false,
        }
    }

    // fn is_token_op(s: &str) -> bool {
    //     match s {
    //         "||" | "&&" | "?" | ":" | "::" |
    //         "+" | "-" | "/" | "*" | "==" | "!=" |
    //         "<=" | ">=" | "<" | ">" => true,
    //         _ => false
    //     }
    // }

    pub fn is_boolean_op(&self) -> bool {
        match &self {
            Operation::Add | Operation::Subtract | Operation::Multiply |
            Operation::Divide | Operation::Negate | Operation::Pow | Operation::Sqrt | 
            Operation::Round | Operation::RoundDown | Operation::RoundUp | Operation::Ternary | 
            Operation::Query | Operation::Find => false,
            Operation::Equal | Operation::NotEqual | Operation::LessThan | 
            Operation::LessThanEq | Operation::GreaterThan | Operation::GreaterThanEq | 
            Operation::Not | Operation::Or | Operation::And => true,
        }
    }

    pub fn get_operator(s: &str, prev: Option<&str>) -> Option<Operation> {
        let mut is_prefix = prev.is_none();
        if let Some(s) = prev {
            is_prefix = !s.starts_with(|c: char| c == ')' || c.is_alphanumeric());
        }
        if is_prefix {
            match s {
                "-" => Some(Operation::Negate),
                "!" => Some(Operation::Not),
                _ => None
            }
        } else {
            match s {
                "sqrt" => Some(Operation::Sqrt),
                "pow" | "^" => Some(Operation::Pow),
                "round" => Some(Operation::Round),
                "rounddown" => Some(Operation::RoundDown),
                "roundup" => Some(Operation::RoundUp),
                "+" => Some(Operation::Add),
                "-" => Some(Operation::Subtract),
                "*" => Some(Operation::Multiply),
                "/" => Some(Operation::Divide),
                "?" => Some(Operation::Ternary),
                "::" => Some(Operation::Query),
                "find" => Some(Operation::Find),
                "==" => Some(Operation::Equal),
                "!=" => Some(Operation::NotEqual),
                "<" => Some(Operation::LessThan),
                "<=" => Some(Operation::LessThanEq),
                ">" => Some(Operation::GreaterThan),
                ">=" => Some(Operation::GreaterThanEq),
                "||" => Some(Operation::Or),
                "&&" => Some(Operation::And),
                _ => None
            }
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
            Operation::Ternary => -1,
            Operation::Query => 3,
            Operation::Find => 3,
            Operation::Equal => 1,
            Operation::NotEqual => 1,
            Operation::LessThan => 1,
            Operation::LessThanEq => 1,
            Operation::GreaterThan => 1,
            Operation::GreaterThanEq => 1,
            Operation::Not => 0,
            Operation::Or => 0,
            Operation::And => 0,
        }
    }
}