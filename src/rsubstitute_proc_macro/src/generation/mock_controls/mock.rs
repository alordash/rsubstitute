use crate::generation::mock_controls::constants::*;
use crate::generation::mock_controls::models::*;
use crate::generation::mock_controls::*;
use crate::syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) fn generate(
    source_span: Span,
    mock_ident: Ident,
    setup_struct_ident: Ident,
    received_struct_ident: Ident,
    data_struct_ident: Ident,
    base_fields: impl IntoIterator<Item = Field>,
) -> Mock {
    let path = path::from_ident(mock_ident.clone());

    let named_fields = [
        Field {
            attrs: Vec::new(),
            vis: Visibility::Public(Token!(pub)(source_span)),
            mutability: FieldMutability::None,
            ident: Some(setup_ident(source_span)),
            colon_token: Some(Token![:](source_span)),
            ty: Type::Path(TypePath {
                qself: None,
                path: path::from_ident(setup_struct_ident),
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
                path: path::from_ident(received_struct_ident),
            }),
        },
        arc_data_field::new(source_span, data_struct_ident),
    ]
    .into_iter()
    .chain(base_fields)
    .collect();

    let item_struct = ItemStruct {
        attrs: Vec::new(),
        vis: Visibility::Public(Token!(pub)(source_span)),
        struct_token: Token![struct](source_span),
        ident: mock_ident,
        generics: Generics::default(),
        fields: Fields::Named(FieldsNamed {
            brace_token: token::Brace(source_span),
            named: named_fields,
        }),
        semi_token: None,
    };

    let result = Mock { path, item_struct };

    return result;
}
