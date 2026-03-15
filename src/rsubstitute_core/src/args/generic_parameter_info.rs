use std::fmt::Debug;

pub enum GenericParameterInfo {
    Type(GenericTypeInfo),
    Const(GenericConstInfo),
}

pub struct GenericTypeInfo {
    pub name: &'static str,
    pub type_name: &'static str,
}

pub struct GenericConstInfo {
    pub name: &'static str,
    pub debug_value_str: String,
}

pub fn generic_type_info<T: ?Sized>(name: &'static str) -> GenericParameterInfo {
    let result = GenericParameterInfo::Type(GenericTypeInfo {
        name,
        type_name: core::any::type_name::<T>(),
    });
    return result;
}

pub fn generic_const_info<T: Debug>(name: &'static str, value: T) -> GenericParameterInfo {
    let result = GenericParameterInfo::Const(GenericConstInfo {
        name,
        debug_value_str: format!("{value:?}"),
    });
    return result;
}
// TODO - code format all project
