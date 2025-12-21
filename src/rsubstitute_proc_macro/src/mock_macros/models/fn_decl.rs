use crate::mock_macros::constants;
use proc_macro2::Ident;
use syn::{FnArg, ReturnType, Type};

pub struct FnDecl {
    pub(crate) ident: Ident,
    pub(crate) arguments: Vec<FnArg>,
    pub(crate) return_value: ReturnType,
}

impl FnDecl {
    pub fn get_return_value_type(&self) -> Type {
        match &self.return_value {
            ReturnType::Default => constants::VOID_TYPE.clone(),
            ReturnType::Type(_, boxed_type) => *boxed_type.clone(),
        }
    }

    pub fn has_return_value(&self) -> bool {
        match self.return_value {
            ReturnType::Default => false,
            ReturnType::Type(_, _) => true,
        }
    }
}
