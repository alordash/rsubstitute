use crate::args_matching::{ArgMatchingResult, IArgsFormatter};

pub trait IArgsChecker<TCall>: IArgsFormatter {
    fn check(&self, call: TCall) -> Vec<ArgMatchingResult>;
}
