use crate::{read::error::{LexerError, OperandLexerError}, token::{Token, operator::executables, parantheses::ParanthesesType}};

#[derive(Debug, PartialEq)]
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
            ' ' => Self::Nothing,
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' | '.' => Self::BuildOperand,
            '+' | '*' | '/' | '^' => Self::BuildOperator,
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

pub fn postfix(infix_equation: &str) -> Result<Vec<Token>, LexerError> {
    let mut buffer: Vec<Token> = Vec::new();
    let mut operators: Vec<Token> = Vec::new();

    let mut builder = String::new();

    let mut prev_state = LexState::Nothing;

    for value in infix_equation.chars() {
        let current_state = LexState::proceed(&prev_state, value);
        match current_state {
            LexState::Nothing => {},
            LexState::Unknown => return Result::Err(LexerError::UnknownNextLexState(prev_state, value)),
            LexState::PushParantheses => {

                if prev_state == LexState::BuildOperand {
                    if !builder.is_empty() {
                        match builder.parse::<f64>()  {
                            Ok(numeric_value) => {
                                buffer.push(Token::Num(numeric_value));
                                builder.clear();
                            },
                            Err(_) => return Result::Err(LexerError::OperandError(OperandLexerError::InvalidNumericOperandConversion(builder))),
                        }
                    }
                    operators.push(Token::Operator(executables::MULTI));
                }

                operators.push(Token::Parentheses(ParanthesesType::Open));
            }

            LexState::SeekParantheses => {
                if !builder.is_empty() {
                    match builder.parse::<f64>()  {
                        Ok(numeric_value) => {
                            buffer.push(Token::Num(numeric_value));
                            builder.clear();
                        },
                        Err(_) => return Result::Err(LexerError::OperandError(OperandLexerError::InvalidNumericOperandConversion(builder))),
                    }
                }

                while !operators.is_empty() {
                    match operators.last() {
                        Option::Some(peek) => {
                            match peek {
                                Token::Operator(_) => {
                                    buffer.push(*peek);
                                    operators.pop();
                                }
                                Token::Parentheses(parantheses_type) => {
                                    if *parantheses_type == ParanthesesType::Open {
                                        operators.pop();
                                        break;
                                    }
                                }
                                _ => return Result::Err(LexerError::OperatorStackCorrupted(*peek))
                            }
                        },
                        Option::None => return Result::Err(LexerError::EmptyOperatorStack)
                    }
                }
            }

            LexState::BuildOperand => {
                if prev_state == LexState::SeekParantheses {
                    operators.push(Token::Operator(executables::MULTI));
                }
                builder.push(value);
            }
            LexState::BuildOperator => {
                if !builder.is_empty() {
                    match builder.parse::<f64>()  {
                        Ok(numeric_value) => {
                            buffer.push(Token::Num(numeric_value));
                            builder.clear();
                        },
                        Err(_) => return Result::Err(LexerError::OperandError(OperandLexerError::InvalidNumericOperandConversion(builder))),
                    }
                }
                while !operators.is_empty() {
                    match operators.last() {
                        Option::None => return Result::Err(LexerError::EmptyOperatorStack),
                        Option::Some(peek) => {
                            match peek {
                                Token::Operator(executable) => {
                                    if executable.prec() >= crate::token::operator::executable::prec(value) {
                                        buffer.push(*peek);
                                        operators.pop();
                                    } else {
                                        break;
                                    }
                                }
                                Token::Parentheses(parantheses_type) => {
                                    if *parantheses_type == ParanthesesType::Open {
                                        break;
                                    }
                                }
                                _ => return Result::Err(LexerError::OperatorStackCorrupted(*peek))
                            }
                        }
                    }
                }
                match executables::match_char_with_executable(value) {
                    Option::Some(executable) => operators.push(Token::Operator(executable)),
                    Option::None => return Result::Err(LexerError::InvalidOperatorDetectedAsOperator(value))
                }
            }
        }
        prev_state = current_state;
    }

    if !builder.is_empty() {
        match builder.parse::<f64>()  {
            Ok(numeric_value) => {
                buffer.push(Token::Num(numeric_value));
                builder.clear();
            },
            Err(_) => return Result::Err(LexerError::OperandError(OperandLexerError::InvalidNumericOperandConversion(builder))),
        }
    }

    while !operators.is_empty() {
        match operators.pop() {
            Option::Some(token) => match token {
                Token::Operator(_) => buffer.push(token),
                _ => return Result::Err(LexerError::OperatorStackCorrupted(token))
            },
            Option::None => return Result::Err(LexerError::EmptyOperatorStack)
        }
    }

    Result::Ok(buffer)
}
