use std::fmt::{Debug, Display, Formatter};

#[derive(Clone)]
pub enum GenericParameterInfo {
    Type(GenericTypeInfo),
    Const(GenericConstInfo),
}

#[derive(Clone)]
pub struct GenericTypeInfo {
    pub name: &'static str,
    pub type_name: &'static str,
}

#[derive(Clone)]
pub struct GenericConstInfo {
    pub name: &'static str,
    pub debug_value_str: String,
}

pub fn generic_type_info(name: &'static str, type_name: &'static str) -> GenericParameterInfo {
    let result = GenericParameterInfo::Type(GenericTypeInfo { name, type_name });
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

impl Display for GenericParameterInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GenericParameterInfo::Type(type_info) => {
                write!(f, "{}", type_info.type_name)
            }
            GenericParameterInfo::Const(const_info) => {
                write!(f, "{}", const_info.debug_value_str)
            }
        }
    }
}
