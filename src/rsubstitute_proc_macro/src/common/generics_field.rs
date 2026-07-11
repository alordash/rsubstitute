use crate::syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) fn new_field(
    span: Span,
    generics: Generics,
    maybe_argument_types: Option<Vec<Type>>,
) -> Field {
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
                [GenericArgument::Type(Type::Tuple(TypeTuple {
                    paren_token: token::Paren(span),
                    elems: generics
                        .params
                        .into_iter()
                        .filter_map(|x| match x {
                            GenericParam::Lifetime(lifetime) => {
                                Some(Type::Reference(TypeReference {
                                    and_token: Token![&](span),
                                    lifetime: Some(lifetime.lifetime),
                                    mutability: None,
                                    elem: Box::new(void_type(span)),
                                }))
                            }
                            GenericParam::Type(ty) => Some(Type::Path(TypePath {
                                qself: None,
                                path: path::from_ident(ty.ident.clone()),
                            })),
                            _ => None,
                        })
                        .chain(
                            maybe_argument_types
                                .into_iter()
                                .flat_map(|arguments| arguments.into_iter()),
                        )
                        .collect(),
                }))],
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
        expr: Expr::Path(ExprPath {
            attrs: Vec::new(),
            qself: None,
            // TODO - replace all other references to STD lib with global paths
            path: path::new_global(span, ["core", "marker", "PhantomData"]),
        }),
    };
    return result;
}
