use crate::token::{Token, operator::{self, executable::{OperatorExecutable, prec}, executables::{self, match_char_with_executable}}, parantheses::ParanthesesType};

pub enum LexState {
    Unknown,
    Nothing,
    BuildOperand,
    BuildOperator,
    PushParantheses,
    SeekParantheses,
}

impl LexState {

    pub fn get(input: char) -> Self {
        match input {
            '1' | '2' | '3' | '4' | '5' |
            '6' | '7' | '8' | '9' | '0' => Self::BuildOperand,
            '+' | '-' | '*' | '/' => Self::BuildOperator,
            '('  => Self::PushParantheses,
            ')' => Self::SeekParantheses,
            _ => Self::Unknown
        }
    }

}


pub fn prec_chars(value: char) -> i32 {
    match value {
        '(' | ')' => -1,
        _ => {
            let precedence = prec(value);
            match precedence {
                _ => precedence as i32
            }
        }
    }
}

pub fn postfix(infix_equation: &str) -> Vec<Token> {
    let mut buffer: Vec<Token> = Vec::new();
    let mut operators: Vec<Token> = Vec::new();

    let mut builder = String::new();
    
    for (index, value) in infix_equation.chars().enumerate() {
        let state = LexState::get(value);
        match state {
            LexState::Unknown | LexState::Nothing => {},
            LexState::PushParantheses => {
                operators.push(Token::Parentheses(ParanthesesType::Open));
            }

            LexState::SeekParantheses => {
                if !builder.is_empty() {
                    let numeric_value = builder.parse().expect("Unable to parse num to f64");
                    buffer.push(Token::Num(numeric_value));
                    builder.clear();
                }
                while !operators.is_empty() {
                    let peek = operators.last().expect("Unable to get last");
                    match peek {
                        Token::Operator(_) => {
                            buffer.push(*peek);
                            operators.pop();
                        }
                        Token::Parentheses(parantheses_type) => {
                            match parantheses_type {
                                ParanthesesType::Open => {
                                    operators.pop();
                                    break;
                                }
                                ParanthesesType::Close => {
                                    panic!("Detected Corrupted Postfix")
                                }
                            }
                        }
                        _ => {
                            panic!("Type is not an operator")
                        }
                    }
                }
            }

            LexState::BuildOperand => {
                builder.push(value);
            },
            LexState::BuildOperator => {
                if !builder.is_empty() {
                    let numeric_value = builder.parse().expect("Unable to parse num to f64");
                    buffer.push(Token::Num(numeric_value));
                    builder.clear();
                }
                while !operators.is_empty() {
                    let peek = operators.last().unwrap(); 
                    match peek {
                        Token::Operator(executable) => {
                            if executable.prec() > prec(value) {
                                buffer.push(*peek);
                                operators.pop();
                            }
                            else {
                                break;
                            }
                        }
                        Token::Parentheses(parantheses_type) => {
                            match parantheses_type {
                                ParanthesesType::Open => {
                                    break;
                                },
                                ParanthesesType::Close => {
                                    panic!("Corrupted operations stack!!");
                                }
                            }
                        }
                        _ => {
                            panic!("Corrupted postfix!!");
                        }
                    }
                }
                operators.push(Token::Operator(executables::match_char_with_executable(value).unwrap()));
            },
        }
    }

    if !buffer.is_empty() {
        let numeric_value = builder.parse().expect("Unable to parse number to a f64");
        buffer.push(Token::Num(numeric_value));
        builder.clear();
    }

    while !operators.is_empty() {
        match operators.pop() {
            Option::Some(token) => {
                match token {
                    Token::Operator(_) => {
                        buffer.push(token);
                    }
                    Token::Parentheses(parantheses) => {
                        match parantheses {
                            ParanthesesType::Open => {
                                panic!("Unclosed parantheses")
                            }
                            ParanthesesType::Close => {
                                panic!("Unopened parantheses")
                            }
                        }
                    }
                    _ => {
                        panic!("Corrupted stack");
                    }
                }
            }
            Option::None => {
                break;
            }
        }
    }

    buffer
}