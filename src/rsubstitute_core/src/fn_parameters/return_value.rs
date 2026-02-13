use crate::IRawReturnValue;
use std::ops::Deref;

pub struct ReturnValue<'a> {
    inner: Box<dyn IRawReturnValue<'a> + 'a>,
}

impl<'a> Clone for ReturnValue<'a> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone_box(),
        }
    }
}

impl<'a> Deref for ReturnValue<'a> {
    type Target = dyn IRawReturnValue<'a> + 'a;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl<'a> ReturnValue<'a> {
    pub fn new<T: IRawReturnValue<'a> + 'a>(raw_return_value: T) -> Self {
        Self {
            inner: Box::new(raw_return_value),
        }
    }

    pub fn downcast_into<T: 'a>(self) -> T {
        let raw_ptr = Box::into_raw(self.inner);
        let t_ptr = raw_ptr as *mut T;
        let t_box = unsafe { Box::from_raw(t_ptr) };
        let t = *t_box;
        return t;
    }
}
