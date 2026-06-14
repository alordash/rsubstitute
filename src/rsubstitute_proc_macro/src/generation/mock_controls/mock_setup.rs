use super::*;
use crate::generation::mock_controls::models::*;
use crate::generation::mock_controls::*;
use crate::syntax::*;
use proc_macro2::Span;
use quote::format_ident;
use syn::*;

pub(crate) fn generate(source_span: Span, target_ident: Ident, data_ident: Ident) -> MockSetup {
    let fields_named = FieldsNamed {
        brace_token: token::Brace(source_span),
        named: punctuated([Field {
            attrs: Vec::new(),
            vis: Visibility::Inherited,
            mutability: FieldMutability::None,
            ident: Some(Ident::new(constants::DATA_FIELD, source_span)),
            colon_token: Some(Token![:](source_span)),
            ty: Type::Path(r#type::arc_of(
                source_span,
                Type::Path(r#type::path::from_ident(data_ident)),
            )),
        }]),
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
    let clone_impl = clone_impl::new(source_span, r#type.clone());

    let result = MockSetup {
        r#type,
        item_struct,
        clone_impl,
        r#impl: todo!(),
    };

    return result;
}
