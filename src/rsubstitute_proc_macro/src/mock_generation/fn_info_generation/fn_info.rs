use crate::mock_generation::fn_info_generation::models::*;
use crate::mock_generation::fn_info_generation::*;
use crate::mock_generation::mock_parts_generation::models::*;
use crate::mock_generation::models::*;
use proc_macro2::Ident;
use quote::format_ident;

pub(crate) fn generate(ctx: &Ctx, fn_decl: FnDecl, mock_type: &MockType) -> FnInfo {
    let call_struct = call_struct::generate(ctx, &fn_decl, &mock_type.generics);
    let args_checker_struct = args_checker_struct::generate(&fn_decl, &mock_type.generics);
    let args_checker_impl = args_checker_trait_impl::generate(
        &call_struct,
        &args_checker_struct,
        fn_decl.get_internal_phantom_types_count() + mock_type.generics.get_phantom_fields_count(),
    );
    let data_field_ident = fn_decl.get_full_ident();
    let fn_info = FnInfo {
        parent: fn_decl,
        call_struct,
        args_checker_struct,
        args_checker_impl,
        data_field_ident,
    };
    return fn_info;
}
