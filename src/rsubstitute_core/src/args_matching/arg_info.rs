pub struct ArgInfo {
    arg_name: &'static str,
    arg_type_name: &'static str,
}

impl ArgInfo {
    pub fn new<T: ?Sized>(arg_name: &'static str) -> Self {
        let arg_type_name = std::any::type_name::<T>();
        return Self {
            arg_name,
            arg_type_name,
        };
    }

    pub fn arg_name(&self) -> &'static str {
        self.arg_name
    }

    pub fn arg_type_name(&self) -> &'static str {
        self.arg_type_name
    }
}
