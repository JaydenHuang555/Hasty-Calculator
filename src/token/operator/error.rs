use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum OperatorError {
    DivideByZero,
}

impl Error for OperatorError {}

impl Display for OperatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            Self::DivideByZero => "Can not divide by Zero!",
        };
        write!(f, "{}", content)
    }
}
