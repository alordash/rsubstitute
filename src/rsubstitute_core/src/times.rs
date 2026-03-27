use std::fmt::{Display, Formatter};

pub enum Times {
    Never,
    Once,
    Exactly(usize),
    #[cfg_attr(
        not(debug_assertions),
        deprecated = "Expecting varying number of calls is not recommended, as it may lead to inconsistent test run results. Determine concrete number of expected calls."
    )]
    Any,
}

impl Times {
    pub fn matches(&self, count: usize) -> bool {
        match self {
            Times::Never => count == 0,
            Times::Once => count == 1,
            Times::Exactly(exact_count) => count == *exact_count,
            Times::Any => true,
        }
    }
}

impl Display for Times {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Times::Never | Times::Exactly(0) => write!(f, "Expected to never receive a call"),
            Times::Once | Times::Exactly(1) => write!(f, "Expected to receive a call exactly once"),
            Times::Exactly(exact_count) => {
                write!(f, "Expected to receive a call {exact_count} times")
            }
            Times::Any => write!(f, "Expected to receive a call any number of times."),
        }
    }
}

// TODO - impl From<usize> for Times? (Exactly)
