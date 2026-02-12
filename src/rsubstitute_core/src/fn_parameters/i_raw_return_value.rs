use std::any::Any;

pub trait IRawReturnValue: Any {
    fn clone_box(&self) -> Box<dyn IRawReturnValue>;
}

impl<T: Any + Clone> IRawReturnValue for T {
    fn clone_box(&self) -> Box<dyn IRawReturnValue> {
        Box::new(self.clone())
    }
}
