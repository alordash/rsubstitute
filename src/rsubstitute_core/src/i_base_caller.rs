use crate::{Call, ReturnValue};

pub trait IBaseCaller {
    fn call_base(&self, call: Call) -> ReturnValue;
}
