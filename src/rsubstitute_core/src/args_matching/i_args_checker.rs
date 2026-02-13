use crate::args_matching::{ArgCheckResult, IArgsFormatter};
use crate::{Call, IGenericsHashKeyProvider};
use std::any::TypeId;

pub trait IArgsChecker<'a>: 'a + IArgsFormatter + IGenericsHashKeyProvider {
    fn check(&self, raw_call: &Call) -> Vec<ArgCheckResult>;
}

pub struct ArgsChecker<'a> {
    inner: Box<dyn IArgsChecker<'a>>,
}

impl<'a> IArgsFormatter for ArgsChecker<'a> {
    fn fmt_args(&self) -> String {
        self.inner.fmt_args()
    }
}

impl<'a> IGenericsHashKeyProvider for ArgsChecker<'a> {
    fn get_generics_type_ids(&self) -> Vec<TypeId> {
        self.inner.get_generics_type_ids()
    }
}

impl<'a> IArgsChecker<'a> for ArgsChecker<'a> {
    fn check(&self, raw_call: &Call) -> Vec<ArgCheckResult> {
        self.inner.check(raw_call)
    }
}

impl<'a> ArgsChecker<'a> {
    pub fn new<T: IArgsChecker<'a>>(raw_args_checker: T) -> Self {
        Self {
            inner: Box::new(raw_args_checker),
        }
    }
}
