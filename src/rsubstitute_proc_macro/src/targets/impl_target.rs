use crate::preparation::r#struct::*;
use syn::*;
use crate::models::Context;

pub(crate) fn handle_impl(ctx: Context, item_impl: ItemImpl) {
    let syntax = prepare_impl_struct_syntax(PrepareImplStructSyntaxArgs {
        attributes: item_impl.attrs,
        generics: item_impl.generics,
        target_type: item_impl.self_ty,
        impl_items: item_impl.items,
    });
}
