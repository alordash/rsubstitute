use crate::IGenericsHashKeyProvider;
use crate::args_matching::IArgInfosProvider;
use std::ptr::NonNull;

pub trait IRawCall: IArgInfosProvider + IGenericsHashKeyProvider {
    fn clone_box_ptr(&self) -> *const dyn IRawCall;
}

impl<T: IArgInfosProvider + IGenericsHashKeyProvider + Clone> IRawCall for T {
    fn clone_box_ptr(&self) -> *const dyn IRawCall {
        let boxed: Box<T> = Box::new(self.clone());
        let boxed_ptr = Box::into_raw(boxed) as *const _ as *const dyn IRawCall;
        return boxed_ptr;
    }
}
