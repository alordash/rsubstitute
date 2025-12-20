use crate::args_matching::ArgMatchingResult;

pub trait IArgsMatcher<TCall> {
    fn matches(&self, call: TCall) -> Vec<ArgMatchingResult>;

    // TODO - should remove something like:
    // "foo == 2, bar – custom predicate, baz – Any"
    fn fmt_args(&self) -> String;
}
