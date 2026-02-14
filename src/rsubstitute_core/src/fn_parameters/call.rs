use crate::fn_parameters::IRawCall;
use std::ops::Deref;

pub struct Call<'a> {
    inner: Box<dyn IRawCall<'a> + 'a>,
}

impl<'a> Clone for Call<'a> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone_box_ptr(),
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
    pub fn new<'b, T: IRawCall<'b> + 'b>(raw_call: T) -> Self {
        unsafe {
            Self {
                inner: std::mem::transmute::<Box<dyn IRawCall<'b>>, Box<dyn IRawCall<'a>>>(
                    Box::new(raw_call),
                ),
            }
        }
    }

    pub fn downcast_ref<T: 'a>(&self) -> &'a T {
        let raw_ref = self.inner.as_ref();
        let t_ref = unsafe { &*(raw_ref as *const _ as *const T) };
        return t_ref;
    }
}
