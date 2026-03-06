use crate::constants;
use syn::*;

#[derive(Copy, Clone)]
pub enum OutputTypeLifetime {
    Derived,
    Default,
}

impl OutputTypeLifetime {
    pub fn get(&self) -> Lifetime {
        match self {
            OutputTypeLifetime::Derived => constants::DERIVED_LIFETIME.clone(),
            OutputTypeLifetime::Default => constants::DEFAULT_ARG_FIELD_LIFETIME.clone(),
        }
    }
}
