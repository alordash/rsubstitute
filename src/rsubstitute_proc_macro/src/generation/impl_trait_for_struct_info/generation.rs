use crate::common::models::Context;
use crate::generation::fn_info;
use crate::generation::impl_trait_for_struct_info::models::*;
use crate::preparation::r#struct::models::*;

pub(crate) fn generate(
    ctx: &Context,
    impl_trait_for_struct_syntax: ImplTraitForStructSyntax,
) -> ImplTraitForStructInfo {
    let result = ImplTraitForStructInfo {
        attributes: impl_trait_for_struct_syntax.attributes,
        modules: impl_trait_for_struct_syntax.modules,
        target_ident: impl_trait_for_struct_syntax.target_ident,
        target_type: impl_trait_for_struct_syntax.target_type,
        trait_path: impl_trait_for_struct_syntax.trait_path,
        generics: impl_trait_for_struct_syntax.generics,
        constants: impl_trait_for_struct_syntax.constants,
        static_fns: impl_trait_for_struct_syntax
            .static_fns
            .into_iter()
            .map(|ordered| ordered.map(|x| fn_info::generate(ctx, x)))
            .collect(),
        associated_fns: impl_trait_for_struct_syntax
            .associated_fns
            .into_iter()
            .map(|ordered| ordered.map(|x| fn_info::generate(ctx, x)))
            .collect(),
    };
    return result;
}
