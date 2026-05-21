use syn::*;
use crate::preparation::r#fn::*;

pub(crate) fn handle_fn(item_fn: ItemFn) {
    let fn_syntax = prepare_fn_syntax(PrepareFnSyntaxArgs {
        attributes: item_fn.attrs,
        visibility: item_fn.vis,
        signature: item_fn.sig,
        is_default: false,
        maybe_base_impl: Some(item_fn.block),
        maybe_owner: None,
    });
}