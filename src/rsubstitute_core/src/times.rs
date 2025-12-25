use std::fmt::{Display, Formatter};

pub enum Times {
    Once,
    Exactly(usize),
    AtLeast(usize),
    AtMost(usize),
}

impl Times {
    pub fn matches(&self, count: usize) -> bool {
        match self {
            Times::Once => count == 1,
            Times::Exactly(exact_count) => count == *exact_count,
            Times::AtLeast(minimum) => count >= *minimum,
            Times::AtMost(maximum) => count <= *maximum,
        }
    }
}

impl Display for Times {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Times::Once | Times::Exactly(1) => write!(f, "exactly once"),
            Times::Exactly(exact_count) => write!(f, "{exact_count} times"),
            Times::AtLeast(minimum) => write!(f, "at least {minimum} times"),
            Times::AtMost(maximum) => write!(f, "at most {maximum} times"),
        }
    }
}
