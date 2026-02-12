use crate::mock_macros::mock_generation::models::MockGenerics;
use syn::*;

#[derive(Clone)]
pub struct MockType {
    pub ident: Ident,
    pub ty: Type,
    pub generics: MockGenerics,
}
