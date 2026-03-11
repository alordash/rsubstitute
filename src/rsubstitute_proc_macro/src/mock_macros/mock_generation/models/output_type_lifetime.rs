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
            OutputTypeLifetime::Derived => constants::DERIVED_LIFETIME.clone(),
            OutputTypeLifetime::Default => constants::DEFAULT_ARG_LIFETIME.clone(),
        }
    }
}
