use super::*;
use crate::preparation::r#fn::models::*;
use proc_macro2::Ident;
use syn::*;

pub(crate) struct FnInfo {
    pub spans: Spans,
    pub attributes: Vec<Attribute>,
    pub source_signature: Box<Signature>,
    pub visibility: Visibility,
    pub merged_generics: Generics,
    pub generics_field: Field,
    pub fn_ident: Ident,
    pub maybe_self_type: Option<Receiver>,
    pub arguments: Vec<Argument>,
    pub arg_refs_tuple: TypeTuple,
    pub maybe_base_impl: Option<Box<Block>>,
    pub return_type: ReturnType,
    pub call_struct: CallStruct,
    pub args_checker_struct: ArgsCheckerStruct,
}
