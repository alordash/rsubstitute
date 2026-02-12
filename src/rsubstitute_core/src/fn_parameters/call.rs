use crate::fn_parameters::IRawCall;
use std::ops::Deref;

pub struct Call<'a> {
    inner: Box<dyn IRawCall<'a> + 'a>,
}

impl<'a> Clone for Call<'a> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone_box(),
        }
    }
}

impl<'a> Deref for Call<'a> {
    type Target = dyn IRawCall<'a> + 'a;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl<'a> Call<'a> {
    pub fn new<T: IRawCall<'a> + 'a>(raw_call: T) -> Self {
        Self {
            inner: Box::new(raw_call),
        }
    }

    pub fn downcast_ref<T: 'a>(&self) -> &T {
        let raw_ref = self.inner.as_ref();
        let t_ref = unsafe { &*(raw_ref as *const _ as *const T) };
        return t_ref;
    }
}
