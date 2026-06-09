use std::{fmt, fmt::Display};

#[derive(Debug, PartialEq)]
pub enum MathToken {
    Number(f64),
    Add,
    Subtract,
    Multiply,
    Divide,
    LeftParen,
    RightParen
}

impl Display for MathToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathToken::Number(val) => write!(f, "{}", val),
            MathToken::Add => write!(f, "+"),
            MathToken::Subtract => write!(f, "-"),
            MathToken::Multiply => write!(f, "*"),
            MathToken::Divide => write!(f, "/"),
            MathToken::LeftParen => write!(f, "("),
            MathToken::RightParen => write!(f, ")"),
        }
    }
}

impl MathToken {
    pub fn precedence(&self) -> u8 {
        match &self {
            MathToken::Add | MathToken::Subtract => 1,
            MathToken::Multiply | MathToken::Divide => 2,
            _ => 0,
        }
    }
}
