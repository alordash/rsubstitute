use crate::syntax::expr;
use proc_macro2::Span;
use syn::*;

pub(crate) fn new(span: Span) -> FieldValue {
    let result = FieldValue {
        attrs: Vec::new(),
        member: Member::Named(Ident::new("generics", span)),
        colon_token: Some(Token![:](span)),
        expr: Expr::Path(expr::path::new(span, ["PhantomData"])),
    };
    return result;
}
