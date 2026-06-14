use crate::generation::mock_controls::*;
use crate::generation::r#fn::*;
use crate::preparation::models::*;
use crate::preparation::r#fn::*;
use syn::spanned::Spanned;
use syn::*;

pub(crate) fn handle(ctx: Context, item_fn: ItemFn) {
    let source_span = item_fn.span();
    let fn_syntax = fn_syntax::prepare(fn_syntax::Params {
        attributes: item_fn.attrs,
        visibility: item_fn.vis,
        signature: item_fn.sig,
        is_default: false,
        maybe_base_impl: Some(item_fn.block),
        maybe_owner: None,
    });
    let fn_info = fn_info::generate(fn_syntax);
    let target_ident = fn_info.syntax.fn_ident.clone();
    let mock_type = mock_type::generate(target_ident.clone());
    let mock_data = mock_data::generate(
        source_span,
        target_ident.clone(),
        mock_type,
        &[fn_info],
        ctx.support_base_calling,
        false,
    );
    todo!()
}
