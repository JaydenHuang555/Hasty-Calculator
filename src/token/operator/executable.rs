use std::fmt::Display;

type Action = fn(f64, f64) -> f64;

pub struct OperatorExecutable<'a> {
    display_value: &'a str,
    action: Action
}

impl<'a> OperatorExecutable<'a> {

    pub const fn new(display_value: &'a str, action: Action) -> Self {
        Self {
            display_value,
            action: action
        }
    }

    pub fn execute(&self, left: f64, right: f64) -> f64 {
        (self.action)(left, right)
    }

}

impl<'a> Display for OperatorExecutable<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_value)
    }
}
