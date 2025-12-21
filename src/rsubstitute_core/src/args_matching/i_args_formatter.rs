pub trait IArgsFormatter {
    // TODO - should remove something like:
    // "foo == 2, bar – custom predicate, baz – Any"
    fn fmt_args(&self) -> String;
}
