use std::{error::Error, fmt::Display};

use crate::{read::lexer::LexState, token::Token};

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
    UnknownNextLexState(LexState, char),
    OperandError(OperandLexerError),
    OperatorStackCorrupted(Token)
}

impl Error for LexerError {}

impl Display for LexerError {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
            Self::UnknownNextLexState(last_output, input) => write!(f, "Unknown next lex state (last output: {:?}, input: {})", last_output, input),
            Self::OperandError(err) => err.fmt(f),
            Self::OperatorStackCorrupted(token) => write!(f, "Operator stack is corrupted with {} detected", token)
       }
    }
}