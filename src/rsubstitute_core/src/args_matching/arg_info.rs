use std::fmt::Debug;

pub struct ArgInfo<'a> {
    arg_name: &'static str,
    arg_type_name: &'static str,
    arg_value: Box<dyn Debug + 'a>,
}

impl<'a> ArgInfo<'a> {
    pub fn new<T: Debug + 'a>(arg_name: &'static str, arg_value: T) -> Self {
        let arg_type_name = std::any::type_name::<T>();
        let arg_value: Box<dyn Debug> = Box::new(arg_value);
        return Self {
            arg_name,
            arg_type_name,
            arg_value,
        };
    }

    pub fn arg_name(&self) -> &'static str {
        self.arg_name
    }

    pub fn arg_type_name(&self) -> &'static str {
        self.arg_type_name
    }

    pub fn arg_value(&self) -> &Box<dyn Debug + 'a> {
        &self.arg_value
    }
}
