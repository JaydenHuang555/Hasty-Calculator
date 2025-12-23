use crate::token::{Token, operator::{executable::OperatorExecutable, executables}, parantheses::ParanthesesType};

#[derive(PartialEq)]
pub enum LexState {
    Unknown,
    Nothing,
    BuildOperand,
    BuildOperator,
    PushParantheses,
    SeekParantheses,
}

impl LexState {
    pub fn proceed(last_output: &LexState, input: char) -> Self {
        match input {
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => Self::BuildOperand,
            '+' | '*' | '/' => Self::BuildOperator,
            '-' => match last_output {
                Self::BuildOperand => Self::BuildOperator,
                _ => Self::BuildOperand,
            },
            '(' => Self::PushParantheses,
            ')' => Self::SeekParantheses,
            _ => Self::Unknown,
        }
    }
}

pub fn postfix(infix_equation: &str) -> Vec<Token> {
    let mut buffer: Vec<Token> = Vec::new();
    let mut operators: Vec<Token> = Vec::new();

    let mut builder = String::new();

    let mut prev_state = LexState::Nothing;

    for value in infix_equation.chars() {
        let current_state = LexState::proceed(&prev_state, value);
        match current_state {
            LexState::Unknown | LexState::Nothing => {}
            LexState::PushParantheses => {

                if prev_state == LexState::BuildOperand {
                    if !builder.is_empty() {
                        let numeric_value = builder.parse().expect("Unable to parse num to f64");
                        buffer.push(Token::Num(numeric_value));
                        builder.clear();
                    }
                    operators.push(Token::Operator(executables::MULTI));
                }

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
                        Token::Parentheses(parantheses_type) => match parantheses_type {
                            ParanthesesType::Open => {
                                operators.pop();
                                break;
                            }
                            ParanthesesType::Close => {
                                panic!("Detected Corrupted Postfix")
                            }
                        },
                        _ => {
                            panic!("Type is not an operator")
                        }
                    }
                }
            }

            LexState::BuildOperand => {
                match prev_state {
                    LexState::SeekParantheses => {
                        operators.push(Token::Operator(executables::MULTI))
                    }
                    _ => {}
                }
                builder.push(value);
            }
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
                            if executable.prec() > crate::token::operator::executable::prec(value) {
                                buffer.push(*peek);
                                operators.pop();
                            } else {
                                break;
                            }
                        }
                        Token::Parentheses(parantheses_type) => match parantheses_type {
                            ParanthesesType::Open => {
                                break;
                            }
                            ParanthesesType::Close => {
                                panic!("Corrupted operations stack!!");
                            }
                        },
                        _ => {
                            panic!("Corrupted postfix!!");
                        }
                    }
                }
                operators.push(Token::Operator(
                    crate::token::operator::executables::match_char_with_executable(value).unwrap(),
                ));
            }
        }
        prev_state = current_state;
    }

    if !buffer.is_empty() {
        let numeric_value = builder.parse().expect("Unable to parse number to a f64");
        buffer.push(Token::Num(numeric_value));
        builder.clear();
    }

    while !operators.is_empty() {
        match operators.pop() {
            Option::Some(token) => match token {
                Token::Operator(_) => {
                    buffer.push(token);
                }
                Token::Parentheses(parantheses) => match parantheses {
                    ParanthesesType::Open => {
                        panic!("Unclosed parantheses")
                    }
                    ParanthesesType::Close => {
                        panic!("Unopened parantheses")
                    }
                },
                _ => {
                    panic!("Corrupted stack");
                }
            },
            Option::None => {
                break;
            }
        }
    }

    buffer
}
