use crate::mock_macros::mock_generation::models::*;
use crate::syntax::*;
use proc_macro2::Ident;

pub(crate) fn generate(mock_ident: Ident, mock_generics: MockGenerics) -> MockType {
    let stores_mock_data = false;
    let result = generate_core(mock_ident, mock_generics, stores_mock_data);
    return result;
}

pub(crate) fn generate_for_struct(mock_ident: Ident, mock_generics: MockGenerics) -> MockType {
    let stores_mock_data = true;
    let result = generate_core(mock_ident, mock_generics, stores_mock_data);
    return result;
}

fn generate_core(
    mock_ident: Ident,
    mock_generics: MockGenerics,
    stores_mock_data: bool,
) -> MockType {
    let ty = r#type::create_with_generics(mock_ident.clone(), mock_generics.impl_generics.clone());
    let mock_type = MockType {
        ident: mock_ident,
        ty,
        generics: mock_generics,
        stores_mock_data,
    };
    return mock_type;
}
