use proc_macro2::Span;
use quote::format_ident;
use syn::*;
use crate::common::*;
use crate::generation::mock_struct::models::*;
use crate::preparation::r#fn::models::*;
use crate::syntax::*;

pub(crate) fn generate(span: Span, fn_syntax: &FnSyntax) -> StaticFnMockStruct {
    let item_struct = ItemStruct {
        attrs: Vec::new(),
        vis: Visibility::Public(Token![pub](span)),
        struct_token: Token![struct](span),
        ident: format_ident!("{}Mock", fn_syntax.fn_ident),
        generics: fn_syntax.merged_generics.clone(),
        fields: Fields::Named(FieldsNamed {
            brace_token: token::Brace(span),
            named: punctuated([generics_field::new_field(
                span,
                fn_syntax.merged_generics.clone(),
                Some(fn_syntax.arguments.iter_generics_style_types().collect()),
            )]),
        }),
        semi_token: None,
    };
    let path = path::from_ident_with_generics(item_struct.ident.clone(), &item_struct.generics);

    let result = StaticFnMockStruct { path, item_struct };
    return result;
}