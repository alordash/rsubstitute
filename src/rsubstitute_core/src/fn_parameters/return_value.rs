use crate::IRawReturnValue;
use std::any::Any;
use std::ops::Deref;

pub struct ReturnValue {
    inner: Box<dyn IRawReturnValue>,
}

impl Clone for ReturnValue {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone_box(),
        }
    }
}

impl Deref for ReturnValue {
    type Target = dyn IRawReturnValue;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl ReturnValue {
    pub fn new<T: IRawReturnValue>(raw_return_value: T) -> Self {
        Self {
            inner: Box::new(raw_return_value),
        }
    }

    pub fn downcast_into<T: 'static>(self) -> T {
        *((self.inner as Box<dyn Any>)
            .downcast()
            .expect("Return type must be TODO"))
    }
}
