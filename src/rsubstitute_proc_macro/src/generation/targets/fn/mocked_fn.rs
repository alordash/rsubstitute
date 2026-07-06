use crate::common::models::*;
use crate::generation::fn_info::models::*;
use crate::generation::mock_struct::models::*;
use crate::generation::mock_struct::*;
use proc_macro2::Span;
use syn::*;

pub(crate) fn generate(
    ctx: &Context,
    source_span: Span,
    fn_info: &FnInfo,
    mock_struct_path: Path,
    maybe_base_fn_ident: Option<Ident>,
) -> ItemFn {
    let block = static_fn_block::generate(
        ctx,
        source_span,
        mock_struct_path,
        fn_info,
        match maybe_base_fn_ident {
            Some(x) => BaseFnKind::Static(x),
            None => BaseFnKind::None,
        },
    );

    let result = ItemFn {
        attrs: fn_info.attributes.clone(),
        vis: Visibility::Public(Token![pub](source_span)),
        sig: *fn_info.source_signature.clone(),
        block: Box::new(block),
    };
    return result;
}
