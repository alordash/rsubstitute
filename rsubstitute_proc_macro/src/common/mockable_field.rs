use crate::syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) fn new_field(span: Span, struct_path: Path) -> Field {
    let result = Field {
        attrs: Vec::new(),
        vis: Visibility::Public(Token![pub](span)),
        modifiers: FieldModifiers::default(),
        ident: Some(Ident::new("mockable", span)),
        colon_token: Some(Token![:](span)),
        ty: Type::Path(r#type::box_of(
            span,
            Type::Path(TypePath {
                attrs: Vec::new(),
                qself: None,
                path: struct_path.clone(),
            }),
        )),
        default: None,
    };
    return result;
}

pub(crate) fn new_value(span: Span, box_new_arg: Expr) -> FieldValue {
    let result = FieldValue {
        attrs: Vec::new(),
        member: Member::Named(Ident::new("mockable", span)),
        colon_token: Some(Token![:](span)),
        expr: Expr::Call(expr::call::new(
            span,
            Expr::Path(ExprPath {
                attrs: Vec::new(),
                qself: None,
                path: path::new(span, ["Box", "new"]),
            }),
            [box_new_arg],
        )),
    };
    return result;
}
