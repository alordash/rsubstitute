use crate::macros::fn_info_generation::call_generator::ICallStructGenerator;
use crate::macros::fn_info_generation::models::FnInfo;
use crate::macros::fn_info_generation::{IArgsMatcherGenerator, IArgsMatcherImplGenerator};
use crate::macros::models::FnDecl;
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
        let args_matcher_impl_info =
            self.args_matcher_impl_generator
                .generate(fn_decl, &call_info, &args_matcher_info);
        let fn_info = FnInfo {
            parent: fn_decl,
            call_info,
            args_matcher_info,
            args_matcher_impl_info,
        };
        return fn_info;
    }
}
