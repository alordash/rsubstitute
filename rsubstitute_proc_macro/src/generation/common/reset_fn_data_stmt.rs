use crate::common::*;
use crate::syntax::*;
use proc_macro2::Span;
use syn::*;

// TODO (DOC) - write in documentation guideline: you should call `setup` or `static_setup` on static fns
// only once in a single unit test, because each `setup` call clears all previous configurations.
pub(crate) fn new(span: Span, mock_generic_argument: GenericArgument) -> ExprCall {
    let result = expr::call::new(
        span,
        Expr::Path(ExprPath {
            attrs: Vec::new(),
            qself: None,
            path: path::new_generics_global(
                span,
                rsubstitute_for_generated::new("clear_static_fn_data"),
                [mock_generic_argument],
            ),
        }),
        [],
    );
    return result;
}
