
use crate::token::operator::{error::OperatorError, executable::{OperatorExecutable, prec}};

pub const ADD: OperatorExecutable =
    OperatorExecutable::new('+', prec('+'), |left, right| -> Result<f64, OperatorError> {
         Result::Ok(left + right) 
    });
pub const SUB: OperatorExecutable =
    OperatorExecutable::new('-', prec('-'), |left, right| -> Result<f64, OperatorError> { Result::Ok(left - right) });

pub const MULTI: OperatorExecutable =
    OperatorExecutable::new('*', prec('*'), |left, right| -> Result<f64, OperatorError> { Result::Ok(left * right) });
pub const DIV: OperatorExecutable =
    OperatorExecutable::new('/', prec('/'), |left, right| -> Result<f64, OperatorError> { 
        match right {
            0.0 => Result::Err(OperatorError::DivideByZero),
            _ => Result::Ok(left / right)
        }
    });

pub const EXPONENT: OperatorExecutable = OperatorExecutable::new('^', prec('^'), |base, exponent| -> Result<f64, OperatorError> {
    match exponent {
        0.0 => Result::Ok(1.0),
        1.0 => Result::Ok(base),
        _ => {
            match exponent.fract() {
                0.0 => Result::Ok(base.powi(exponent as i32)),
                _ => Result::Ok(base.powf(exponent))
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
