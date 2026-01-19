use crate::mock_macros::fn_info_generation::call_generator::*;
use crate::mock_macros::fn_info_generation::models::*;
use crate::mock_macros::fn_info_generation::*;
use crate::mock_macros::mock_generation::models::*;
use crate::mock_macros::models::FnDecl;
use proc_macro2::Ident;
use quote::format_ident;
use std::sync::Arc;

pub trait IFnInfoGenerator {
    fn generate<'a>(&self, fn_decl: &'a FnDecl, mock_generics: &MockGenerics) -> FnInfo<'a>;
}

pub struct FnInfoGenerator {
    pub(crate) call_struct_generator: Arc<dyn ICallStructGenerator>,
    pub(crate) call_arg_infos_provider_impl_generator: Arc<dyn ICallArgInfosProviderImplGenerator>,
    pub(crate) args_checker_generator: Arc<dyn IArgsCheckerGenerator>,
    pub(crate) args_checker_impl_generator: Arc<dyn IArgsCheckerTraitImplGenerator>,
}

impl IFnInfoGenerator for FnInfoGenerator {
    fn generate<'a>(&self, fn_decl: &'a FnDecl, mock_generics: &MockGenerics) -> FnInfo<'a> {
        let call_struct = self.call_struct_generator.generate(fn_decl, mock_generics);
        let call_arg_infos_provider_impl = self
            .call_arg_infos_provider_impl_generator
            .generate(&call_struct);
        let args_checker_struct = self.args_checker_generator.generate(fn_decl, mock_generics);
        let args_checker_impl = self
            .args_checker_impl_generator
            .generate(&call_struct, &args_checker_struct);
        let data_field_ident = self.generate_data_field_ident(fn_decl);
        let fn_info = FnInfo {
            parent: fn_decl,
            call_struct,
            call_arg_infos_provider_impl,
            args_checker_struct,
            args_checker_impl,
            data_field_ident,
        };
        return fn_info;
    }
}

impl FnInfoGenerator {
    const DATA_FIELD_IDENT_SUFFIX: &'static str = "data";

    fn generate_data_field_ident(&self, fn_decl: &FnDecl) -> Ident {
        let data_field_ident = format_ident!("{}_{}", fn_decl.ident, Self::DATA_FIELD_IDENT_SUFFIX);
        return data_field_ident;
    }
}
