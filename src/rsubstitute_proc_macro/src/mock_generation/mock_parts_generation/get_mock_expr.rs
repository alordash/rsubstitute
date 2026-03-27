use crate::constants;
use crate::syntax::*;
use syn::*;

pub(crate) fn generate(generics: Generics) -> Expr {
    let func = path::create_expr_with_generics(constants::GET_MOCK_FN_IDENT.clone(), generics);
    let get_mock_expr = call::create_without_args(func);
    return get_mock_expr;
}
