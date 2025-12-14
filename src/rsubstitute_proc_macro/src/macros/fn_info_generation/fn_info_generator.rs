use crate::macros::fn_info_generation::call_generator::ICallStructGenerator;
use crate::macros::fn_info_generation::{IArgsMatcherGenerator, IArgsMatcherImplGenerator};
use crate::macros::models::FnInfo;
use proc_macro2::TokenStream;
use std::rc::Rc;

pub trait IFnInfoGenerator {
    fn generate(&self, fn_info: &FnInfo) -> TokenStream;
}

pub struct FnInfoGenerator {
    pub(crate) call_struct_generator: Rc<dyn ICallStructGenerator>,
    pub(crate) args_matcher_generator: Rc<dyn IArgsMatcherGenerator>,
    pub(crate) args_matcher_impl_generator: Rc<dyn IArgsMatcherImplGenerator>,
}

impl IFnInfoGenerator for FnInfoGenerator {
    fn generate(&self, fn_info: &FnInfo) -> TokenStream {
        let call_info = self.call_struct_generator.generate(fn_info);
        let args_matcher_info = self.args_matcher_generator.generate(fn_info);
        let args_matcher_impl_info = self
            .args_matcher_impl_generator
            .generate(&call_info, &args_matcher_info);
        todo!()
    }
}
