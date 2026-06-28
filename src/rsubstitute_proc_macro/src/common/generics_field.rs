use crate::syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) fn new_field(span: Span, generics: &Generics) -> Field {
    let result = Field {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        mutability: FieldMutability::None,
        ident: Some(generics_field_ident(span)),
        colon_token: Some(Token![:](span)),
        ty: Type::Path(TypePath {
            qself: None,
            path: path::new_generics(
                span,
                ["PhantomData"],
                GenericArgument::Type(Type::Tuple(TypeTuple {
                    paren_token: token::Paren(span),
                    elems: generics
                        .type_params()
                        .map(|x| {
                            Type::Path(TypePath {
                                qself: None,
                                path: path::from_ident(x.ident.clone()),
                            })
                        })
                        .collect(),
                })),
            ),
        }),
    };
    return result;
}


pub(crate) fn new_value(span: Span) -> FieldValue {
    let result = FieldValue {
        attrs: Vec::new(),
        member: Member::Named(Ident::new("generics", span)),
        colon_token: Some(Token![:](span)),
        expr: Expr::Path(expr::path::new(span, ["PhantomData"])),
    };
    return result;
}