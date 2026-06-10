use crate::syntax::self_path;
use syn::*;

pub(crate) fn new_self(field_ident: Ident) -> ExprField {
    let span = field_ident.span();
    let result = ExprField {
        attrs: Vec::new(),
        base: Box::new(Expr::Path(self_path(span))),
        dot_token: Token![.](span),
        member: Member::Named(field_ident),
    };

    return result;
}
