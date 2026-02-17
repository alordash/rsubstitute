use crate::{Call, ReturnValue};

pub trait IBaseCaller<'a, TCall: 'a> {
    fn dyn_call_base(&self, dyn_call: Call<'a>) -> ReturnValue<'_> {
        let call: &'a TCall = dyn_call.downcast_ref();
        return self.call_base(call);
    }

    fn call_base(&'_ self, call: &TCall) -> ReturnValue<'_>;
}
