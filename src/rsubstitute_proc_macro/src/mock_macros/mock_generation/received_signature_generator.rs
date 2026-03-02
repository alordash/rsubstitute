use crate::constants;
use crate::mock_macros::fn_info_generation::models::*;
use crate::mock_macros::mock_generation::IInputArgsGenerator;
use crate::mock_macros::mock_generation::models::*;
use crate::syntax::{IReferenceNormalizer, ITypeFactory};
use proc_macro2::Ident;
use quote::format_ident;
use std::cell::LazyCell;
use std::iter;
use std::sync::Arc;
use syn::*;

pub trait IReceivedSignatureGenerator {
    fn get_times_arg_ident(&self) -> Ident;

    fn generate_for_trait(&self, fn_info: &FnInfo, mock_type: &MockType) -> Signature;

    fn generate_for_static(
        &self,
        fn_info: &FnInfo,
        mock_received_struct: &MockReceivedStruct,
        mock_type: &MockType,
    ) -> Signature;
}

pub(crate) struct ReceivedSignatureGenerator {
    pub type_factory: Arc<dyn ITypeFactory>,
    pub input_args_generator: Arc<dyn IInputArgsGenerator>,
    pub reference_normalizer: Arc<dyn IReferenceNormalizer>,
}

impl IReceivedSignatureGenerator for ReceivedSignatureGenerator {
    fn get_times_arg_ident(&self) -> Ident {
        format_ident!("times")
    }

    fn generate_for_trait(&self, fn_info: &FnInfo, mock_type: &MockType) -> Signature {
        let result = self.generate(
            fn_info,
            fn_info.parent.fn_ident.clone(),
            constants::SELF_TYPE.clone(),
            MockGenericsUsage::JustGetPhantomTypesCount,
        );
        return result;
    }

    fn generate_for_static(
        &self,
        fn_info: &FnInfo,
        mock_received_struct: &MockReceivedStruct,
        mock_type: &MockType,
    ) -> Signature {
        let mut return_ty = self
            .type_factory
            .create_from_struct(&mock_received_struct.item_struct);
        self.reference_normalizer
            .staticify_anonymous_lifetimes(&mut return_ty);
        let result = self.generate(
            fn_info,
            constants::MOCK_RECEIVED_FIELD_IDENT.clone(),
            return_ty,
            MockGenericsUsage::UseAsGenerics(&mock_type.generics),
        );
        return result;
    }
}

impl ReceivedSignatureGenerator {
    const TIMES_TYPE_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("Times"));

    fn generate(
        &self,
        fn_info: &FnInfo,
        fn_ident: Ident,
        return_ty: Type,
        mock_generics_usage: MockGenericsUsage,
    ) -> Signature {
        let times_arg = FnArg::Typed(PatType {
            attrs: Vec::new(),
            pat: Box::new(Pat::Ident(PatIdent {
                attrs: Vec::new(),
                by_ref: None,
                mutability: None,
                ident: self.get_times_arg_ident(),
                subpat: None,
            })),
            colon_token: Default::default(),
            ty: Box::new(self.type_factory.create(Self::TIMES_TYPE_IDENT.clone())),
        });
        let mut inputs: Vec<_> = self
            .input_args_generator
            .generate_input_args(fn_info)
            .into_iter()
            .chain(iter::once(times_arg))
            .collect();
        let generics = match mock_generics_usage {
            MockGenericsUsage::JustGetPhantomTypesCount => Generics::default(),
            MockGenericsUsage::UseAsGenerics(mock_generics) => mock_generics.impl_generics.clone(),
        };
        let signature = Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: Default::default(),
            ident: fn_ident,
            generics,
            paren_token: Default::default(),
            inputs: inputs.into_iter().collect(),
            variadic: None,
            output: ReturnType::Type(Default::default(), Box::new(return_ty)),
        };
        return signature;
    }
}

enum MockGenericsUsage<'a> {
    JustGetPhantomTypesCount,
    UseAsGenerics(&'a MockGenerics),
}
