use crate::generation::r#fn::models::*;
use crate::preparation::r#fn::models::*;
use quote::format_ident;
use syn::*;

pub(crate) fn generate_args_checker_struct(fn_syntax: &FnSyntax) -> ArgsCheckerStruct {
    let span = fn_syntax.spans.inputs;
    let item_struct = ItemStruct {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        struct_token: Token![struct](span),
        ident: format_ident!("{}_ArgsChecker", fn_syntax.fn_ident),
        generics: fn_syntax.merged_generics.clone(),
        fields,
        semi_token: None,
    };

    let result = ArgsCheckerStruct {};

    return result;
}
