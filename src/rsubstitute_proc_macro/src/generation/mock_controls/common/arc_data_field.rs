use crate::generation::mock_controls::constants::*;
use crate::syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) fn new(span: Span, mock_data_ident: Ident) -> Field {
    let result = Field {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        mutability: FieldMutability::None,
        ident: Some(data_ident(span)),
        colon_token: Some(Token![:](span)),
        ty: Type::Path(r#type::arc_of(
            span,
            Type::Path(TypePath {
                qself: None,
                path: path::from_ident(mock_data_ident),
            }),
        )),
    };

    return result;
}
