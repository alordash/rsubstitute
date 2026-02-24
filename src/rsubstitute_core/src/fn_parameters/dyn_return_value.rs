use crate::fn_parameters::IReturnValue;

pub struct DynReturnValue<'rs> {
    inner: Box<dyn IReturnValue<'rs> +'rs>
}

impl<'rs> DynReturnValue<'rs> {
    pub fn new<T: IReturnValue<'rs> + 'rs>(value: T) -> Self {
        Self {
            inner: Box::new(value)
        }
    }
    
    pub fn downcast_into<T: IReturnValue<'rs> + 'rs>(self) -> T {
        let raw_ptr = Box::into_raw(self.inner) as *mut T;
        let boxed = unsafe {Box::from_raw(raw_ptr)};
        let value = *boxed;
        return value;
    }
}

impl<'rs> Clone for DynReturnValue<'rs> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone_box()
        }
    }
}