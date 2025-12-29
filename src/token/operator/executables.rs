
use crate::token::operator::executable::{OperatorExecutable, prec};

pub const ADD: OperatorExecutable =
    OperatorExecutable::new('+', prec('+'), |left, right| -> f64 { left + right });
pub const SUB: OperatorExecutable =
    OperatorExecutable::new('-', prec('-'), |left, right| -> f64 { left - right });

pub const MULTI: OperatorExecutable =
    OperatorExecutable::new('*', prec('*'), |left, right| -> f64 { left * right });
pub const DIV: OperatorExecutable =
    OperatorExecutable::new('/', prec('/'), |left, right| -> f64 { left / right });

pub const EXPONENT: OperatorExecutable = OperatorExecutable::new('^', prec('^'), |base, exponent| -> f64 {
    match exponent {
        0.0 => 1.0,
        1.0 => base,
        _ => {
            match exponent.fract() {
                0.0 => base.powi(exponent as i32),
                _ => base.powf(exponent)
            }
        }
    }
});

pub const fn match_char_with_executable(value: char) -> Option<OperatorExecutable> {
    match value {
        '+' => Option::Some(ADD),
        '-' => Option::Some(SUB),
        '*' => Option::Some(MULTI),
        '/' => Option::Some(DIV),
        '^' => Option::Some(EXPONENT),
        _ => Option::None,
    }
}
