use crate::token::Token;

pub fn eval(buffer: &Vec<Token>) -> f64 {
    let mut stack: Vec<f64> = Vec::new();

    for token in buffer {
        match token {
            Token::Num(numeric_val) => {
                stack.push(*numeric_val);
            }
            Token::Operator(executable) => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(executable.execute(left, right));
            }
            _ => {
                panic!("Found error with token {}", token);
            }
        }
    }

    stack.pop().unwrap()
}
