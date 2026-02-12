use crate::Call;
use crate::args_matching::{ArgCheckResult, IArgsFormatter};

pub trait IArgsChecker<'a>: 'a + IArgsFormatter {
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
