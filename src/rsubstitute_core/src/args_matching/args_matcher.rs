use crate::args_matching::ArgMatchingResult;
use std::fmt::Debug;

pub trait IArgsMatcher<TCall>: Debug {
    fn matches(&self, call: TCall) -> Vec<Option<ArgMatchingResult>>;
}
