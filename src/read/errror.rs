use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum OperandLexerError {
    InvalidNumericOperandConversion(String)
}

impl Error for OperandLexerError {}

impl Display for OperandLexerError {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidNumericOperandConversion(stored) => write!(f, "Unable to convert {} to a numeric operand!", stored)
        }
    }

}

#[derive(Debug)]
pub enum LexerError {
    OperandError(OperandLexerError)
}

impl Error for LexerError {}

impl Display for LexerError {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
            Self::OperandError(err) => err.fmt(f)
       }
    }
}