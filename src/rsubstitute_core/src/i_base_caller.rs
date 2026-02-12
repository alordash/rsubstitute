use crate::fn_parameters::IRawCall;

pub trait IBaseCaller<TReturnValue> {
    fn call_base(&self, call: Box<dyn IRawCall>) -> TReturnValue;
}
