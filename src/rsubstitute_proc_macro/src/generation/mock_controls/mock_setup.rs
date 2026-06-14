use crate::generation::mock_controls::models::*;
use crate::syntax::*;
use proc_macro2::Span;
use quote::format_ident;
use syn::*;

pub(crate) fn generate(source_span: Span, target_ident: Ident, data_ident: Ident) -> MockSetup {
    let fields_named = FieldsNamed {
        brace_token: token::Brace(source_span),
        named: [Field {
            attrs: Vec::new(),
            vis: Visibility::Inherited,
            mutability: FieldMutability::None,
            ident: Some(format_ident!("data")),
            colon_token: Some(Token![:](source_span)),
            ty: Type::Path(r#type::arc_of(
                source_span,
                Type::Path(r#type::path::from_ident(data_ident)),
            )),
        }]
        .into_iter()
        .collect(),
    };

    let item_struct = ItemStruct {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        struct_token: Token![struct](source_span),
        ident: format_ident!("{target_ident}Setup"),
        generics: Generics::default(),
        fields: Fields::Named(fields_named),
        semi_token: None,
    };

    let r#type = Type::Path(r#type::path::from_ident(item_struct.ident.clone()));

    let result = MockSetup {
        r#type,
        item_struct,
        clone_impl: todo!(),
        r#impl: todo!(),
    };

    return result;
}
