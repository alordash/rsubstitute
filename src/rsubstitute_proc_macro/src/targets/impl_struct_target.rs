use crate::preparation::r#struct::*;
use syn::*;

pub(crate) fn handle_impl_struct(item_impl: ItemImpl) {
    let syntax = prepare_impl_struct_syntax(PrepareImplStructSyntaxArgs {
        attributes: item_impl.attrs,
        generics: item_impl.generics,
        self_ty: item_impl.self_ty,
        impl_items: item_impl.items,
    });
}
