use crate::constants;
use syn::*;

#[derive(Copy, Clone)]
pub enum OutputTypeLifetime {
    Derived,
    Default,
}

impl OutputTypeLifetime {
    pub(crate) fn get(&self) -> Lifetime {
        match self {
            OutputTypeLifetime::Derived => constants::ANONYMOUS_LIFETIME.clone(),
            OutputTypeLifetime::Default => constants::ANONYMOUS_LIFETIME.clone(), // TODO - what should it be?
        }
    }
}
