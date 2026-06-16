use crate::generation::anonymous_lifetime;
use crate::preparation::r#fn::models::*;
use proc_macro2::Span;
use syn::*;

pub(crate) fn new(span: Span, arguments: &[Argument]) -> TypeTuple {
    let result = TypeTuple {
        paren_token: token::Paren(span),
        elems: arguments
            .iter()
            .map(|argument| {
                Type::Reference(TypeReference {
                    and_token: Token![&](span),
                    lifetime: Some(anonymous_lifetime::new(span)),
                    mutability: None,
                    elem: Box::new(*argument.ref_style_type.clone()),
                })
            })
            .collect(),
    };

    return result;
}
