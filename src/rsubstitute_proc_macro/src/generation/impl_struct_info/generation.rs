use crate::common::models::Context;
use crate::generation::fn_info;
use crate::generation::impl_struct_info::models::*;
use crate::preparation::r#struct::models::*;

pub(crate) fn generate(ctx: &Context, impl_struct_syntax: ImplStructSyntax) -> ImplStructInfo {
    let result = ImplStructInfo {
        attributes: impl_struct_syntax.attributes,
        modules: impl_struct_syntax.modules,
        target_ident: impl_struct_syntax.target_ident,
        target_type: impl_struct_syntax.target_type,
        generics: impl_struct_syntax.generics,
        constants: impl_struct_syntax.constants,
        static_fns: impl_struct_syntax
            .static_fns
            .into_iter()
            .map(|ordered| ordered.map(|x| fn_info::generate(ctx, x)))
            .collect(),
        associated_fns: impl_struct_syntax
            .associated_fns
            .into_iter()
            .map(|ordered| ordered.map(|x| fn_info::generate(ctx, x)))
            .collect(),
    };
    return result;
}
