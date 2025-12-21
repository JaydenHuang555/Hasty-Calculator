use crate::token::operator::executable::OperatorExecutable;

pub const ADD: OperatorExecutable = OperatorExecutable::new("+", |left, right| -> f64 {left + right});
pub const SUB: OperatorExecutable = OperatorExecutable::new("-", |left, right| -> f64 {left - right});

pub const MULTI: OperatorExecutable = OperatorExecutable::new("*", |left, right| -> f64 {left * right});
pub const DIV: OperatorExecutable = OperatorExecutable::new("/", |left, right| -> f64 {left / right});