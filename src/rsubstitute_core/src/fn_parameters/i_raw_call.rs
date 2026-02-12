use crate::args_matching::IArgInfosProvider;
use std::any::Any;

pub trait IRawCall: Any + IArgInfosProvider {
    fn clone_box(&self) -> Box<dyn IRawCall>;
}

impl<T: Any + IArgInfosProvider + Clone> IRawCall for T {
    fn clone_box(&self) -> Box<dyn IRawCall> {
        Box::new(self.clone())
    }
}
