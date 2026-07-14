use crate::common::generics_field;
use crate::generation::common::data_field;
use crate::syntax::*;
use proc_macro2::Span;
use quote::format_ident;
use syn::*;

pub(crate) fn generate(span: Span, struct_ident: Ident, generics: Generics) -> ItemStruct {
    let ident = format_ident!("{struct_ident}Mock");
    let path = path::from_ident_with_generics(struct_ident, &generics);
    let result = ItemStruct {
        attrs: Vec::new(),
        vis: Visibility::Public(Token![pub](span)),
        struct_token: Token![struct](span),
        ident,
        generics: generics.clone(),
        fields: Fields::Named(FieldsNamed {
            brace_token: token::Brace(span),
            named: punctuated([
                generics_field::new_field(span, generics, None),
                data_field::new_field(span, data_field::Params { public: true }),
                Field {
                    attrs: Vec::new(),
                    vis: Visibility::Inherited,
                    mutability: FieldMutability::None,
                    ident: Some(Ident::new("mockable", span)),
                    colon_token: Some(Token![:](span)),
                    ty: Type::Path(r#type::box_of(
                        span,
                        Type::Path(TypePath { qself: None, path }),
                    )),
                },
            ]),
        }),
        semi_token: None,
    };
    return result;
}
