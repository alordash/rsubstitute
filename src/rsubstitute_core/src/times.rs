use std::fmt::{Display, Formatter};

pub enum Times {
    Never,
    Once,
    Exactly(usize),
}

impl Times {
    pub fn matches(&self, count: usize) -> bool {
        match self {
            Times::Never => count == 0,
            Times::Once => count == 1,
            Times::Exactly(exact_count) => count == *exact_count,
        }
    }
}

impl Display for Times {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Times::Never | Times::Exactly(0) => write!(f, "never"),
            Times::Once | Times::Exactly(1) => write!(f, "exactly once"),
            Times::Exactly(exact_count) => write!(f, "{exact_count} times"),
        }
    }
}
