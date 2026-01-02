pub trait IBaseCaller<TCall, TReturnValue> {
    fn new() -> Self;
    
    fn call_base(&self, call: TCall) -> TReturnValue;
}