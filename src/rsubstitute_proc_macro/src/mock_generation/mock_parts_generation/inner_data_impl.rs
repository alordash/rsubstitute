use crate::mock_generation::mock_parts_generation::models::*;
use crate::syntax::generics;
use syn::*;

pub(crate) fn generate(inner_data_struct: &InnerDataStruct, new_fn: ImplItemFn) -> InnerDataImpl {
    let self_ty = inner_data_struct.ty.clone();
    let item = ImplItem::Fn(new_fn);
    let item_impl = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Default::default(),
        generics: generics::remove_default_values(inner_data_struct.item_struct.generics.clone()),
        trait_: None,
        self_ty: Box::new(self_ty),
        brace_token: Default::default(),
        items: vec![item],
    };
    let inner_data_impl = InnerDataImpl { item_impl };
    return inner_data_impl;
}
