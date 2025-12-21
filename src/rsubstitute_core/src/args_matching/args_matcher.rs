use crate::args_matching::{ArgMatchingResult, IArgsFormatter};

pub trait IArgsMatcher<TCall>: IArgsFormatter {
    fn matches(&self, call: TCall) -> Vec<ArgMatchingResult>;
}
