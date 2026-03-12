use crate::mock_macros::fn_info_generation::*;
use crate::mock_macros::mock_generation::models::*;
use crate::mock_macros::models::*;
use crate::mock_macros::*;

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
    let mock_struct_trait_info = MockStructTraitInfo {
        trait_ident_from_path,
        mock_type: mock_type.clone(),
        fn_infos,
    };
    return mock_struct_trait_info;
}
