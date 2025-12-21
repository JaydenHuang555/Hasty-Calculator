use std::fmt::Display;

use crate::token::operator::executable::OperatorExecutable;

pub mod operator;

pub enum Token {
    Num(f64),
    Operator(OperatorExecutable)
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Num(operand) => write!(f, "{}", operand),
            Self::Operator(executable) => write!(f, "{}", executable)
        }
    }
}