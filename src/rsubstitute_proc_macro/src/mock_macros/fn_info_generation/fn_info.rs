use crate::mock_macros::fn_info_generation::models::*;
use crate::mock_macros::fn_info_generation::*;
use crate::mock_macros::mock_generation::models::*;
use crate::mock_macros::models::*;
use proc_macro2::Ident;
use quote::format_ident;

pub(crate) fn generate(ctx: &Ctx, fn_decl: FnDecl, mock_type: &MockType) -> FnInfo {
    let call_struct = call_struct::generate(ctx, &fn_decl, &mock_type.generics);
    let args_checker_struct = args_checker::generate(&fn_decl, &mock_type.generics);
    let args_checker_impl = args_checker_trait_impl::generate(
        &call_struct,
        &args_checker_struct,
        fn_decl.get_internal_phantom_types_count() + mock_type.generics.get_phantom_fields_count(),
    );
    let data_field_ident = generate_data_field_ident(&fn_decl);
    let fn_info = FnInfo {
        parent: fn_decl,
        call_struct,
        args_checker_struct,
        args_checker_impl,
        data_field_ident,
    };
    return fn_info;
}

const DATA_FIELD_IDENT_SUFFIX: &'static str = "data";

fn generate_data_field_ident(fn_decl: &FnDecl) -> Ident {
    let data_field_ident =
        format_ident!("{}_{}", fn_decl.get_full_ident(), DATA_FIELD_IDENT_SUFFIX);
    return data_field_ident;
}
