use crate::token::operator::executable::OperatorExecutable;

pub mod operator;

pub enum Token<'a> {
    Num(f64),
    Operator(OperatorExecutable<'a>)
}