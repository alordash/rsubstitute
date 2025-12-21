use crate::args_matching::{ArgMatchingResult, IArgsFormatter};

pub trait IArgsChecker<TCall>: IArgsFormatter {
    fn matches(&self, call: TCall) -> Vec<ArgMatchingResult>;
}
