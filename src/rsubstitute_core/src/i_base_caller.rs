pub trait IBaseCaller<TCall, TReturnValue> {
    fn call_base(&self, call: TCall) -> TReturnValue;
}
