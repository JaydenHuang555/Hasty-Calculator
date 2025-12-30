use crate::{eval::error::EvaluateError, token::Token};

pub fn evaluate(buffer: &Vec<Token>) -> Result<f64, EvaluateError> {
    let mut stack: Vec<f64> = Vec::new();

    for token in buffer {
        match token {
            Token::Num(numeric_val) => {
                stack.push(*numeric_val);
            }
            Token::Operator(executable) => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                match executable.execute(left, right) {
                    Result::Ok(output) => {
                        stack.push(output)
                    },
                    Result::Err(err) => {
                        return Result::Err(EvaluateError::OperationError(err));
                    }
                }
            }
            _ => {
                return Result::Err(EvaluateError::InvalidToken(*token));
            }
        }
    }

    match stack.last() {
        Option::None => Result::Err(EvaluateError::NoPeek),
        Option::Some(operand) => Result::Ok(*operand)
    }
}
