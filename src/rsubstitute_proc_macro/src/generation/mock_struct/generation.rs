use crate::common::*;
use crate::generation::mock_struct::models::*;
use crate::preparation::r#fn::models::*;
use crate::syntax::*;
use proc_macro2::Span;
use quote::format_ident;
use syn::*;

pub(crate) fn generate_for_static_fn(source_span: Span, fn_syntax: &FnSyntax) -> MockStruct {
    let item_struct = ItemStruct {
        attrs: Vec::new(),
        vis: Visibility::Public(Token![pub](source_span)),
        struct_token: Token![struct](source_span),
        ident: format_ident!("{}Mock", fn_syntax.fn_ident),
        generics: fn_syntax.merged_generics.clone(),
        fields: Fields::Named(FieldsNamed {
            brace_token: token::Brace(source_span),
            named: punctuated([generics_field::new_field(
                source_span,
                &fn_syntax.merged_generics,
            )]),
        }),
        semi_token: None,
    };
    let path = path::from_ident(item_struct.ident.clone());

    let result = MockStruct { path, item_struct };
    return result;
}
