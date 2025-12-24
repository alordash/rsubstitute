use crate::args_matching::{ArgCheckResult, IArgsFormatter};

pub trait IArgsChecker<TCall>: IArgsFormatter {
    fn check(&self, call: TCall) -> Vec<ArgCheckResult>;
}
