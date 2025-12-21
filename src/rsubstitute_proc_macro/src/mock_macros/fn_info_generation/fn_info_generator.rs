use crate::mock_macros::fn_info_generation::call_generator::ICallStructGenerator;
use crate::mock_macros::fn_info_generation::models::FnInfo;
use crate::mock_macros::fn_info_generation::{IArgsMatcherGenerator, IArgsMatcherImplGenerator};
use crate::mock_macros::models::FnDecl;
use proc_macro2::Ident;
use quote::format_ident;
use std::rc::Rc;

pub trait IFnInfoGenerator {
    fn generate<'a>(&self, fn_decl: &'a FnDecl) -> FnInfo<'a>;
}

pub struct FnInfoGenerator {
    pub(crate) call_struct_generator: Rc<dyn ICallStructGenerator>,
    pub(crate) args_matcher_generator: Rc<dyn IArgsMatcherGenerator>,
    pub(crate) args_matcher_impl_generator: Rc<dyn IArgsMatcherImplGenerator>,
}

impl IFnInfoGenerator for FnInfoGenerator {
    fn generate<'a>(&self, fn_decl: &'a FnDecl) -> FnInfo<'a> {
        let call_info = self.call_struct_generator.generate(fn_decl);
        let args_matcher_info = self.args_matcher_generator.generate(fn_decl);
        let args_matcher_impl_info = self
            .args_matcher_impl_generator
            .generate(&call_info, &args_matcher_info);
        let data_field_ident = self.generate_data_field_ident(fn_decl);
        let fn_info = FnInfo {
            parent: fn_decl,
            call_info,
            args_matcher_info,
            args_matcher_impl_info,
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
