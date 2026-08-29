#[derive(PartialEq, Debug)]
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

pub trait ITimes: Sized {
    // Syntactic sugar for writing `1.time()`
    fn time(self) -> Times {
        Self::times(self)
    }

    fn times(self) -> Times;
}

impl ITimes for usize {
    fn times(self) -> Times {
        Times::Exactly(self)
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    fn ITimes_times_Ok() {
        // Arrange
        let raw = 1usize;

        // Act
        let times = raw.times();

        // Assert
        assert_eq!(Times::Exactly(1), times);
    }
}
