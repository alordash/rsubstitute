use crate::preparation::r#fn::models::*;
use crate::syntax::path;
use proc_macro2::Span;
use syn::*;

pub(crate) fn generate_args_provider_impl(
    arguments: &[Argument],
    target_type: Type,
    span: Span,
) -> ItemImpl {
    let items = todo!();

    let result = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Token![impl](span),
        generics: Generics::default(),
        trait_: Some((None, path::new(["IArgsProvider"], span), Token![for](span))),
        self_ty: Box::new(target_type),
        brace_token: token::Brace(span),
        items,
    };

    return result;
}
