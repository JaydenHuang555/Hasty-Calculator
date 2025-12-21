use std::fmt::Display;

use crate::token::{operator::executable::OperatorExecutable, parantheses::ParanthesesType};

pub mod operator;
pub mod parantheses;

pub enum Token {
    Num(f64),
    Operator(OperatorExecutable),
    Parentheses(ParanthesesType)
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Num(operand) => write!(f, "{}", operand),
            Self::Operator(executable) => write!(f, "{}", executable),
            Self::Parentheses(parantheses_type) => parantheses_type.fmt(f)
        }
    }
}