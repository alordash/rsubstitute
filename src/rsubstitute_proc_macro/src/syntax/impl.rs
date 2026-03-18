use crate::mock_generation::mock_parts_generation::models::*;
use syn::*;

pub(crate) fn create_with_default_lifetime(
    mock_type: &MockType,
    self_ty: Type,
    items: Vec<ImplItem>,
) -> ItemImpl {
    let item_impl = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Default::default(),
        generics: mock_type.generics.impl_generics_without_default_values.clone(),
        trait_: None,
        self_ty: Box::new(self_ty),
        brace_token: Default::default(),
        items,
    };
    return item_impl;
}
