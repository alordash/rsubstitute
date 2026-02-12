use crate::fn_parameters::IRawCall;
use std::ops::Deref;

pub struct Call {
    inner: Box<dyn IRawCall>,
}

impl Clone for Call {
    fn clone(&self) -> Self {
        Call {
            inner: self.inner.clone_box(),
        }
    }
}

impl Deref for Call {
    type Target = dyn IRawCall;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl Call {
    pub fn new<T: IRawCall>(raw_call: T) -> Self {
        Self {
            inner: Box::new(raw_call),
        }
    }

    pub fn into_raw_call(self) -> Box<dyn IRawCall> {
        self.inner
    }
}
