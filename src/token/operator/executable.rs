use std::fmt::Display;

type Action = fn(f64, f64) -> f64;

pub const fn prec(value: char) -> u32 {
    match value {
        '+' | '-' => 1,
        '*' | '/' | '%' => 2,
        '^' => 3,
        _ => 0
    } 
}

pub struct OperatorExecutable {
    display_value: char,
    prec: u32,
    action: Action
}

impl OperatorExecutable {

    pub const fn new(display_value: char, prec: u32, action: Action) -> Self {
        Self {
            display_value,
            prec,
            action: action
        }
    }

    pub fn execute(&self, left: f64, right: f64) -> f64 {
        (self.action)(left, right)
    }

    pub fn prec(&self) -> u32 {
        self.prec
    }

}

impl Display for OperatorExecutable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_value)
    }
}
