pub trait IBaseCaller<TMock, TCall, TReturnValue> {
    fn call_base(&self, mock: &TMock, call: TCall) -> TReturnValue;
}