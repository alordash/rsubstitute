use crate::args::*;
use crate::mock_data::*;
use crate::*;
use std::ops::Deref;
use std::ptr::NonNull;

pub struct DynFnDataHolder {
    ptr: NonNull<dyn IDynFnData>,
}

impl DynFnDataHolder {
    pub fn new<TMock, TCall, TReturnType, TArgsChecker>(fn_name: &'static str) -> Self
    where
        TCall: IGenericsHashKeyProvider + IArgInfosProvider,
        TArgsChecker: IArgsChecker<TCall>,
        TReturnType: Clone,
    {
        let fn_data = FnData::<TMock, TCall, TReturnType, TArgsChecker>::new("work", &SERVICES);
        let boxed: Box<dyn IDynFnData> = Box::new(fn_data);
        let raw_ptr = Box::into_raw(boxed);
        let local_ptr = unsafe { NonNull::new_unchecked(raw_ptr) };
        let ptr = unsafe { std::mem::transmute(local_ptr) };
        Self { ptr }
    }
}

impl Deref for DynFnDataHolder {
    type Target = dyn IDynFnData;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}
