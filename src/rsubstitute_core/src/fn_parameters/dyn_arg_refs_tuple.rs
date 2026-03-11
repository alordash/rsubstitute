use crate::fn_parameters::{IArgRefsTuple, IReturnValue};

pub(crate) struct DynArgRefsTuple<'rs> {
    inner: Box<dyn IArgRefsTuple<'rs> + 'rs>,
}

impl<'rs> DynArgRefsTuple<'rs> {
    pub fn from_raw(raw_ptr: *mut (dyn IArgRefsTuple<'rs> + 'rs)) -> Self {
        Self {
            // SAFETY: for justification refer to module level documentation.
            inner: unsafe { Box::from_raw(raw_ptr) },
        }
    }

    pub fn downcast_into<'a, T: IReturnValue<'a>>(self) -> T {
        let raw_ptr = Box::into_raw(self.inner) as *mut T;
        // SAFETY: for justification refer to module level documentation.
        let boxed = unsafe { Box::from_raw(raw_ptr) };
        let value = *boxed;
        return value;
    }
}
