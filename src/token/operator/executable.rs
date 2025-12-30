use std::{error::Error, fmt::Display};

use crate::token::operator::error::OperatorError;

type Action = fn(f64, f64) -> Result<f64, OperatorError>;

pub const fn prec(value: char) -> u32 {
    match value {
        '+' | '-' => 1,
        '*' | '/' | '%' => 2,
        '^' => 3,
        _ => 0,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OperatorExecutable {
    display_value: char,
    prec: u32,
    action: Action,
}

impl OperatorExecutable {
    pub const fn new(display_value: char, prec: u32, action: Action) -> Self {
        Self {
            display_value,
            prec,
            action,
        }
    }

    pub fn execute(&self, left: f64, right: f64) -> Result<f64, OperatorError> {
        (self.action)(left, right)
    }

    pub fn prec(&self) -> u32 {
        self.prec
    }
}

impl Display for OperatorExecutable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_value)
    }
}
