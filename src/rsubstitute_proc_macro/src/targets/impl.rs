use crate::preparation::models::*;
use crate::preparation::r#struct::*;
use syn::*;

pub(crate) fn handle(ctx: Context, item_impl: ItemImpl) {
    let syntax = impl_struct_syntax::prepare(impl_struct_syntax::Params {
        attributes: item_impl.attrs,
        generics: item_impl.generics,
        target_type: item_impl.self_ty,
        impl_items: item_impl.items,
    });
    todo!()
}
