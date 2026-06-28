use crate::generation::mock_struct::models::*;
use crate::preparation::r#fn::models::*;
use crate::syntax::*;
use proc_macro2::Span;
use quote::format_ident;
use syn::*;

pub(crate) fn generate_for_static_fn(source_span: Span, fn_syntax: &FnSyntax) -> MockStruct {
    let item_struct = ItemStruct {
        attrs: Vec::new(),
        vis: Visibility::Public(Token!(pub)(source_span)),
        struct_token: Token![struct](source_span),
        ident: format_ident!("{}Mock", fn_syntax.fn_ident),
        generics: fn_syntax.merged_generics.clone(),
        fields: Fields::Named(FieldsNamed {
            brace_token: token::Brace(source_span),
            named: punctuated([Field {
                attrs: Vec::new(),
                vis: Visibility::Inherited,
                mutability: FieldMutability::None,
                ident: Some(generics_field_ident(source_span)),
                colon_token: Some(Token![:](source_span)),
                ty: Type::Path(TypePath {
                    qself: None,
                    path: path::new_generics(
                        source_span,
                        ["PhantomData"],
                        GenericArgument::Type(Type::Tuple(TypeTuple {
                            paren_token: token::Paren(source_span),
                            elems: fn_syntax
                                .merged_generics
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
            }]),
        }),
        semi_token: None,
    };
    let path = path::from_ident(item_struct.ident.clone());

    let result = MockStruct { path, item_struct };
    return result;
}
