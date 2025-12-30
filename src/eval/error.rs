use std::{error::Error, fmt::Display};

use crate::token::{Token, operator::error::OperatorError};

#[derive(Debug)]
pub enum EvaluateError {
    OperationError(OperatorError),
    NoPeek,
    InvalidToken(Token),
}

impl Error for EvaluateError {}

impl Display for EvaluateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OperationError(operator_error) => operator_error.fmt(f),
            Self::NoPeek => write!(f, "No peek operand found. Internal Error"),
            Self::InvalidToken(token) => {
                write!(f, "Invalid Token {} found. Error when lexing.", token)
            }
        }
    }
}
