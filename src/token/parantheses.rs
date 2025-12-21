use std::fmt::Display;

#[derive(Clone, Copy)]
pub enum ParanthesesType {
    Open,
    Close,
}

impl Display for ParanthesesType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Open => "(",
                Self::Close => ")",
            }
        )
    }
}
