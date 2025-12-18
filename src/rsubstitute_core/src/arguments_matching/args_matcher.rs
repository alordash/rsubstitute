pub trait IArgsMatcher<TCall> {
    fn matches(&self, call: TCall) -> Vec<Option<String>>;
}
