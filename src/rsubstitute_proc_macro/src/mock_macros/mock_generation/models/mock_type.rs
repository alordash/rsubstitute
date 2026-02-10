use crate::mock_macros::mock_generation::models::MockGenerics;
use syn::*;

#[derive(Clone)]
pub struct MockType {
    pub ident: Ident,
    // TODO - use it instead of manually creating mock type each time
    pub ty: Type,
    pub generics: MockGenerics,
}
