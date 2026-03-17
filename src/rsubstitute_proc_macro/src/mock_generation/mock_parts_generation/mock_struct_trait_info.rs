use crate::mock_generation::fn_info_generation::*;
use crate::mock_generation::mock_parts_generation::models::*;
use crate::mock_generation::models::*;
use crate::mock_generation::*;
use syn::*;

pub(crate) fn generate(
    ctx: &Ctx,
    mock_type: &MockType,
    trait_impl: TraitImpl,
) -> MockStructTraitInfo {
    let trait_ident_from_path = trait_impl.get_trait_ident_from_path();
    let fn_decls = fn_decl::extract_struct_trait_impl_fns(ctx, &mock_type.generics, &trait_impl);
    let fn_infos: Vec<_> = fn_decls
        .into_iter()
        .map(|fn_decl| fn_info::generate(ctx, fn_decl, &mock_type))
        .collect();
    let rest_impl_items = trait_impl
        .item_impl
        .items
        .into_iter()
        .filter(is_rest_impl_item)
        .collect();
    let mock_struct_trait_info = MockStructTraitInfo {
        trait_ident_from_path,
        mock_type: mock_type.clone(),
        fn_infos,
        rest_impl_items,
    };
    return mock_struct_trait_info;
}

fn is_rest_impl_item(impl_item: &ImplItem) -> bool {
    match impl_item {
        ImplItem::Fn(_) => false,
        _ => true,
    }
}
