use crate::args_matching::{ArgCheckResult, IArgsFormatter};

pub trait IArgsChecker<TCall>: IArgsFormatter {
    fn check(&'_ self, call: TCall) -> Vec<ArgCheckResult<'_>>;
}
