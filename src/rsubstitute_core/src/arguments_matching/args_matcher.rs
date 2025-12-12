pub trait IArgsMatcher<TCall> {
    fn matches(&self, call: TCall) -> bool;
}
