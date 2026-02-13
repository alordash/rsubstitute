use crate::{Call, ReturnValue};

pub trait IBaseCaller {
    fn call_base(&'_ self, call: Call) -> ReturnValue<'_>;
}
