use std::ptr::NonNull;

// Using manual casting because what is passed and
// what is expected both controlled by generated code.
pub struct DynFnDataParam {
    ptr: NonNull<()>,
}

impl DynFnDataParam {
    pub fn new<T>(value: T) -> Self {
        let boxed = Box::new(value);
        let raw_ptr = Box::into_raw(boxed) as *mut ();
        let ptr = unsafe { NonNull::new_unchecked(raw_ptr) };
        return Self { ptr };
    }

    pub fn downcast_into<T>(self) -> T {
        let boxed = unsafe { Box::from_raw(self.ptr.as_ptr() as *mut T) };
        return *boxed;
    }
}

pub trait IIntoDynFnDataParam {
    fn upcast_into(self) -> DynFnDataParam;
}

impl<T> IIntoDynFnDataParam for T {
    fn upcast_into(self) -> DynFnDataParam {
        DynFnDataParam::new(self)
    }
}