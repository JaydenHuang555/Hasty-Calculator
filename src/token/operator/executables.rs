use crate::token::operator::executable::{OperatorExecutable, prec};

pub const ADD: OperatorExecutable =
    OperatorExecutable::new('+', prec('+'), |left, right| -> f64 { left + right });
pub const SUB: OperatorExecutable =
    OperatorExecutable::new('-', prec('-'), |left, right| -> f64 { left - right });

pub const MULTI: OperatorExecutable =
    OperatorExecutable::new('*', prec('*'), |left, right| -> f64 { left * right });
pub const DIV: OperatorExecutable =
    OperatorExecutable::new('/', prec('/'), |left, right| -> f64 { left / right });

pub const fn match_char_with_executable(value: char) -> Option<OperatorExecutable> {
    match value {
        '+' => Option::Some(ADD),
        '-' => Option::Some(SUB),
        '*' => Option::Some(MULTI),
        '/' => Option::Some(DIV),
        _ => Option::None,
    }
}
