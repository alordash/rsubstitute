use crate::generation::mock_controls::{generate_mock_data, generate_mock_type};
use crate::generation::r#fn::*;
use crate::preparation::models::*;
use crate::preparation::r#fn::*;
use syn::spanned::Spanned;
use syn::*;

pub(crate) fn handle_fn(ctx: Context, item_fn: ItemFn) {
    let source_span = item_fn.span();
    let fn_syntax = prepare_fn_syntax(PrepareFnSyntaxArgs {
        attributes: item_fn.attrs,
        visibility: item_fn.vis,
        signature: item_fn.sig,
        is_default: false,
        maybe_base_impl: Some(item_fn.block),
        maybe_owner: None,
    });
    let fn_info = generate_fn_info(fn_syntax);
    let target_ident = fn_info.syntax.fn_ident.clone();
    let mock_type = generate_mock_type(target_ident.clone());
    let mock_data = generate_mock_data(
        source_span,
        target_ident.clone(),
        mock_type,
        &[fn_info],
        ctx.support_base_calling,
        false,
    );
    todo!()
}
