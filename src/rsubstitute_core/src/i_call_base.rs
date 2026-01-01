pub trait ICallBase<TCall, TReturnValue> {
    fn new() -> Self;
    
    fn call_base(&self, call: TCall) -> TReturnValue;
}