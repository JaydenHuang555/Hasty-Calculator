use crate::token::{Token, operator::{self, executable::{OperatorExecutable, prec}, executables::match_char_with_executable}};

pub fn prec_chars(value: char) -> u32 {
    let operator_prec = prec(value);
    match operator_prec {
        0 => {
            0
        }
        _ => {
            operator_prec
        }
    }
}

pub fn postfix(infix_equation: &str) -> Vec<Token> {
    let mut buffer = Vec::new();
    let mut operators: Vec<OperatorExecutable> = Vec::new();

    let mut builder = String::new();
    
    for (index, value) in infix_equation.chars().enumerate() {
        match prec_chars(value) {
            0 => {
                builder.push(value);

            },
            _ => {
                if !builder.is_empty() {
                    let numeric_value: f64= builder.parse().expect("Unable to parse a number to a f64");
                    buffer.push(Token::Num(numeric_value));
                    builder.clear();
                }
                while !operators.is_empty() {
                    let peek = operators.last().expect("Corruption found in the stack operator buffer");
                    if peek.prec() > prec(value) {
                        buffer.push(Token::Operator(operators.pop().unwrap()));
                    }
                    else {
                        break;
                    }
                }
                operators.push(match_char_with_executable(value).unwrap());
            }
        }
    }

    if !buffer.is_empty() {
        let numeric_value = builder.parse().expect("Unable to parse number to a f64");
        buffer.push(Token::Num(numeric_value));
        builder.clear();
    }

    while !operators.is_empty() {
        buffer.push(Token::Operator(operators.pop().unwrap()));
    }

    buffer
}