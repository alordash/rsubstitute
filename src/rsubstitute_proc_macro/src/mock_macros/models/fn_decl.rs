use crate::constants;
use proc_macro2::Ident;
use syn::*;

pub(crate) struct FnDecl {
    pub ident: Ident,
    pub arguments: Vec<FnArg>,
    pub return_value: ReturnType,
    pub maybe_base_fn_block: Option<Block>,
    pub maybe_parent_trait_path: Option<Path>
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
