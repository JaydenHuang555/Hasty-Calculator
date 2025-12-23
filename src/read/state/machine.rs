use crate::read::state::{prepare::PrepareLexState, proceed::ProceedLexState};

pub struct LexStateOutput {
    pub prepare: PrepareLexState,
    pub proceed: ProceedLexState
}

impl Default for LexStateOutput {
    fn default() -> Self {
        Self {
            prepare: PrepareLexState::default(),
            proceed: ProceedLexState::default()
        }
    }
}

pub struct LexStateMachine {
    last_output: LexStateOutput
}

impl LexStateMachine {

    pub fn new() -> Self {
        Self {
            last_output: LexStateOutput::default() 
        }
    }

    pub fn update(&mut self, last_output: LexStateOutput) {
        self.last_output = last_output;
    }

    pub fn prepare(&mut self, input: char) -> PrepareLexState {
        match input {
            '('  => {
                match self.last_output.proceed {
                    ProceedLexState::BuildOperand => {
                        
                    }
                    _ => PrepareLexState::Nothing
                }
            }
            _ => PrepareLexState::Nothing
        }
    }

    pub fn proceed(&mut self, input: char) -> ProceedLexState {
        match input {
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => ProceedLexState::BuildOperand,
            '+' | '*' | '/' => ProceedLexState::BuildOperator,
            '-' => {
                match self.last_output.proceed {
                    ProceedLexState::BuildOperand => ProceedLexState::BuildOperator,
                    _ => ProceedLexState::BuildOperand
                }
            }
            '(' => ProceedLexState::PushParantheses,
            ')' => ProceedLexState::SeekParantheses,
            _ => ProceedLexState::Unknown,
        }
    }

}