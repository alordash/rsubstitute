use crate::common::generics_field;
use crate::generation::mock_struct::models::*;
use crate::preparation::r#trait::models::*;
use crate::syntax::*;
use proc_macro2::Span;
use quote::format_ident;
use syn::*;

pub(crate) fn generate(span: Span, trait_syntax: TraitSyntax) -> TraitMockStruct {
    let item_struct = ItemStruct {
        attrs: Vec::new(),
        vis: Visibility::Public(Token![pub](span)),
        struct_token: Token![struct](span),
        ident: format_ident!("{}Mock", trait_syntax.ident),
        generics: trait_syntax.merged_generics.clone(),
        fields: Fields::Named(FieldsNamed {
            brace_token: token::Brace(span),
            named: punctuated([generics_field::new_field(
                span,
                trait_syntax.merged_generics,
                None,
            )]),
        }),
        semi_token: None,
    };
    let path = path::from_ident_with_generics(item_struct.ident.clone(), &item_struct.generics);

    let result = TraitMockStruct {
        path,
        item_struct,
        trait_impl: todo!(),
        inner_impl: todo!(),
    };
    return result;
}
