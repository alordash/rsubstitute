use crate::macros::fn_info_generation::call_generator::ICallStructGenerator;
use crate::macros::models::FnInfo;
use proc_macro2::TokenStream;
use std::rc::Rc;

pub trait IFnInfoGenerator {
    fn generate(&self, fn_info: &FnInfo) -> TokenStream;
}

pub struct FnInfoGenerator {
    pub(crate) call_struct_generator: Rc<dyn ICallStructGenerator>,
}

impl IFnInfoGenerator for FnInfoGenerator {
    fn generate(&self, fn_info: &FnInfo) -> TokenStream {
        let call_struct = self.call_struct_generator.generate(fn_info);
        todo!()
    }
}
