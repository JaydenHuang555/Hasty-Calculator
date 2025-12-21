use std::fmt::Display;

use crate::token::{operator::executable::OperatorExecutable, parantheses::ParanthesesType};

pub mod operator;
pub mod parantheses;

#[derive(Clone, Copy)]
pub enum Token {
    Num(f64),
    Operator(OperatorExecutable),
    Parentheses(ParanthesesType),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Num(operand) => operand.fmt(f),
            Self::Operator(executable) => executable.fmt(f),
            Self::Parentheses(parantheses_type) => parantheses_type.fmt(f),
        }
    }
}
