use std::fmt::Debug;

pub struct ArgInfo {
    arg_name: &'static str,
    arg_type_name: &'static str,
    arg_value_ptr: *const dyn Debug,
}

impl ArgInfo {
    pub fn new<T: Debug>(arg_name: &'static str, arg_value: &T) -> Self {
        let arg_type_name = std::any::type_name::<T>();
        let arg_value_ptr = std::ptr::from_ref(arg_value) as *const dyn Debug;
        return Self {
            arg_name,
            arg_type_name,
            arg_value_ptr,
        };
    }

    pub fn arg_name(&self) -> &'static str {
        self.arg_name
    }

    pub fn arg_type_name(&self) -> &'static str {
        self.arg_type_name
    }

    pub fn arg_value_debug_str(&self) -> String {
        unsafe { format!("{:?}", &*self.arg_value_ptr) }
    }
}
