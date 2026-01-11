use crate::constants;
use crate::mock_macros::fn_info_generation::models::*;
use crate::mock_macros::mock_generation::models::*;
use crate::mock_macros::mock_generation::IInputArgsGenerator;
use crate::syntax::{IReferenceNormalizer, ITypeFactory};
use proc_macro2::Ident;
use quote::format_ident;
use std::cell::LazyCell;
use std::iter;
use std::sync::Arc;
use syn::*;

pub trait IReceivedSignatureGenerator {
    fn get_times_arg_ident(&self) -> Ident;

    fn generate_for_trait(&self, fn_info: &FnInfo) -> Signature;

    fn generate_for_static(
        &self,
        fn_info: &FnInfo,
        mock_received_struct: &MockReceivedStruct,
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

    fn generate_for_trait(&self, fn_info: &FnInfo) -> Signature {
        let return_ty = Type::Reference(TypeReference {
            and_token: Default::default(),
            lifetime: Some(constants::DEFAULT_ARG_FIELD_LIFETIME.clone()),
            mutability: None,
            elem: Box::new(constants::SELF_TYPE.clone()),
        });
        let prepend_ref_self_arg = true;
        let result = self.generate(
            fn_info,
            fn_info.parent.ident.clone(),
            prepend_ref_self_arg,
            return_ty,
        );
        return result;
    }

    fn generate_for_static(
        &self,
        fn_info: &FnInfo,
        mock_received_struct: &MockReceivedStruct,
    ) -> Signature {
        let mut return_ty = self
            .type_factory
            .create_from_struct(&mock_received_struct.item_struct);
        self.reference_normalizer.staticify(&mut return_ty);
        let return_ty_reference = Type::Reference(TypeReference {
            and_token: Default::default(),
            lifetime: Some(constants::STATIC_LIFETIME.clone()),
            mutability: None,
            elem: Box::new(return_ty),
        });
        let prepend_ref_self_arg = false;
        let mut result = self.generate(
            fn_info,
            constants::MOCK_RECEIVED_FIELD_IDENT.clone(),
            prepend_ref_self_arg,
            return_ty_reference,
        );
        for input in result.inputs.iter_mut() {
            if let FnArg::Typed(pat_type) = input {
                self.reference_normalizer.staticify(pat_type.ty.as_mut());
            }
        }
        return result;
    }
}

impl ReceivedSignatureGenerator {
    const TIMES_TYPE_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("Times"));

    fn generate(
        &self,
        fn_info: &FnInfo,
        fn_ident: Ident,
        prepend_ref_self_arg: bool,
        return_ty: Type,
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
        if prepend_ref_self_arg {
            inputs.insert(0, constants::REF_SELF_ARG_WITH_LIFETIME.clone());
        }
        let signature = Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: Default::default(),
            ident: fn_ident,
            generics: Generics::default(),
            paren_token: Default::default(),
            inputs: inputs.into_iter().collect(),
            variadic: None,
            output: ReturnType::Type(Default::default(), Box::new(return_ty)),
        };
        return signature;
    }
}
