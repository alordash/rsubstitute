use crate::mock_generation::mock_parts_generation::models::MockGenerics;
use syn::*;

#[derive(Clone)]
pub(crate) struct MockType {
    pub ident: Ident,
    pub ty_path: TypePath,
    pub generics: MockGenerics,
    pub stores_mock_data: bool
}
