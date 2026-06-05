use crate::models::r#fn::Argument;
use syn::*;

pub(crate) struct FnSyntax {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub merged_generics: Generics,
    pub fn_ident: Ident,
    pub is_default: bool,
    pub maybe_self_type: Option<Receiver>,
    pub arguments: Vec<Argument>,
    pub return_type: ReturnType,
    pub maybe_base_impl: Option<Box<Block>>,
}
