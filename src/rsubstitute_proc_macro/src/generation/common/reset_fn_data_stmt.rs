use crate::syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) fn new(span: Span, mock_generic_argument: GenericArgument) -> ExprCall {
    let result = expr::call::new(
        span,
        Expr::Path(ExprPath {
            attrs: Vec::new(),
            qself: None,
            path: path::new_generics_global(
                span,
                ["rsubstitute", "for_generated", "clear_static_fn_data"],
                mock_generic_argument,
            ),
        }),
        [],
    );
    return result;
}
