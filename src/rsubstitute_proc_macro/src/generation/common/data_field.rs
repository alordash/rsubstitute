use crate::syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) struct Params {
    pub public: bool,
}
pub(crate) fn new_field(span: Span, Params { public }: Params) -> Field {
    let result = Field {
        attrs: Vec::new(),
        vis: if public {
            Visibility::Public(Token![pub](span))
        } else {
            Visibility::Inherited
        },
        mutability: FieldMutability::None,
        ident: Some(Ident::new("data", span)),
        colon_token: Some(Token![:](span)),
        ty: Type::Path(TypePath {
            qself: None,
            path: Path {
                leading_colon: Some(Token![::](span)),
                segments: rsubstitute_punctuated(
                    span,
                    [PathSegment {
                        ident: Ident::new("SharedMockData", span),
                        arguments: PathArguments::None,
                    }],
                ),
            },
        }),
    };
    return result;
}

pub(crate) fn new_default_value(span: Span) -> FieldValue {
    let result = FieldValue {
        attrs: Vec::new(),
        member: Member::Named(Ident::new("data", span)),
        colon_token: Some(Token![:](span)),
        expr: Expr::Call(expr::call::new(
            span,
            Expr::Path(ExprPath {
                attrs: Vec::new(),
                qself: None,
                path: path::new_global(span, ["core", "default", "Default", "default"]),
            }),
            [],
        )),
    };
    return result;
}

pub(crate) fn new_clone_value(span: Span) -> FieldValue {
    let result = FieldValue {
        attrs: Vec::new(),
        member: Member::Named(Ident::new("data", span)),
        colon_token: Some(Token![:](span)),
        expr: Expr::MethodCall(expr::method_call::new(
            span,
            Expr::Field(expr::field::new(
                Expr::Path(self_expr_path(span)),
                Ident::new("data", span),
            )),
            Ident::new("clone", span),
            [],
        )),
    };
    return result;
}
