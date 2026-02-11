use crate::mock_macros::mock_generation::models::*;
use quote::format_ident;
use syn::*;

pub trait IInnerDataParamGenerator {
    fn generate<'a>(
        &'a self,
        inner_data_struct: &'a InnerDataStruct,
        new_fn: &ImplItemFn,
    ) -> InnerDataParam<'a>;
}

pub(crate) struct InnerDataParamGenerator;

impl IInnerDataParamGenerator for InnerDataParamGenerator {
    fn generate<'a>(
        &'a self,
        inner_data_struct: &'a InnerDataStruct,
        new_fn: &ImplItemFn,
    ) -> InnerDataParam<'a> {
        let inner_data_param = InnerDataParam {
            inner_data_struct: &inner_data_struct,
            constructor_arguments: new_fn
                .sig
                .inputs
                .iter()
                .enumerate()
                .map(|(i, x)| {
                    let FnArg::Typed(pat_type) = x else {
                        panic!("Mock struct constructor arguments should all be typed.")
                    };
                    let ident = match &*pat_type.pat {
                        Pat::Ident(pat_ident) => pat_ident.ident.clone(),
                        _ => format_ident!("arg{i}"),
                    };
                    return (ident, (*pat_type.ty).clone());
                })
                .collect(),
        };
        return inner_data_param;
    }
}
