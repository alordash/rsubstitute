use crate::mock_macros::mock_generation::models::MockGenerics;
use syn::*;

pub struct MockType {
    pub ident: Ident,
    pub generics: MockGenerics,
}
