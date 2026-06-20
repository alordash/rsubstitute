mod mock_struct_impls;

use crate::generation::mock_controls::constants::*;
use crate::generation::mock_controls::*;
use crate::generation::models::*;
use crate::syntax::*;
use proc_macro2::Span;
use quote::format_ident;
use syn::*;

// TODO - separately generate mock_impl - its what generated for static fn
// and what's generated for #[mock] on struct impl
pub(crate) fn generate(
    source_span: Span,
    target_ident: Ident,
    setup_struct_ident: Ident,
    received_struct_ident: Ident,
    data_struct_ident: Ident,
    maybe_target_struct: Option<ItemStruct>,
) -> Mock {
    let ident = format_ident!("{target_ident}Mock");
    let path = path::from_ident(ident.clone());

    let named_fields = punctuated([
        Field {
            attrs: Vec::new(),
            vis: Visibility::Public(Token!(pub)(source_span)),
            mutability: FieldMutability::None,
            ident: Some(setup_ident(source_span)),
            colon_token: Some(Token![:](source_span)),
            ty: Type::Path(TypePath {
                qself: None,
                path: path::from_ident(setup_struct_ident.clone()),
            }),
        },
        Field {
            attrs: Vec::new(),
            vis: Visibility::Public(Token!(pub)(source_span)),
            mutability: FieldMutability::None,
            ident: Some(received_ident(source_span)),
            colon_token: Some(Token![:](source_span)),
            ty: Type::Path(TypePath {
                qself: None,
                path: path::from_ident(received_struct_ident.clone()),
            }),
        },
        arc_data_field::new(source_span, data_struct_ident.clone()),
    ]);

    let item_struct = ItemStruct {
        attrs: Vec::new(),
        vis: Visibility::Public(Token!(pub)(source_span)),
        struct_token: Token![struct](source_span),
        ident,
        generics: Generics::default(),
        fields: Fields::Named(FieldsNamed {
            brace_token: token::Brace(source_span),
            named: named_fields,
        }),
        semi_token: None,
    };

    let maybe_mock_struct_impls = maybe_target_struct
        .map(|target_struct| {
            mock_struct_impls::generate(
                source_span,
                item_struct.ident.clone(),
                target_struct,
                setup_struct_ident,
                received_struct_ident,
                data_struct_ident,
            )
        })
        .map(Box::new);

    let result = Mock {
        path,
        item_struct,
        maybe_mock_struct_impls,
    };

    return result;
}
