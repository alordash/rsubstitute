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

pub(crate) trait IReceivedSignatureGenerator {
    fn get_times_arg_ident(&self) -> Ident;

    fn generate_for_trait(
        &self,
        fn_info: &FnInfo,
        mock_type: &MockType,
        output_type_generics: OutputTypeGenerics,
    ) -> Signature;

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

    fn generate_for_trait(
        &self,
        fn_info: &FnInfo,
        mock_type: &MockType,
        output_type_generics: OutputTypeGenerics,
    ) -> Signature {
        let prepend_ref_self_arg = true;
        let result = self.generate(
            fn_info,
            fn_info.parent.fn_ident.clone(),
            prepend_ref_self_arg,
            constants::SELF_TYPE.clone(),
            mock_type,
            output_type_generics,
        );
        return result;
    }

    fn generate_for_static(
        &self,
        fn_info: &FnInfo,
        mock_received_struct: &MockReceivedStruct,
        mock_type: &MockType,
    ) -> Signature {
        let mut owner_type = self
            .type_factory
            .create_from_struct(&mock_received_struct.item_struct);
        self.reference_normalizer
            .staticify_anonymous_lifetimes(&mut owner_type);
        let prepend_ref_self_arg = false;
        let result = self.generate(
            fn_info,
            constants::MOCK_RECEIVED_FIELD_IDENT.clone(),
            prepend_ref_self_arg,
            owner_type,
            mock_type,
            OutputTypeGenerics::UseMock,
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
        prepend_ref_self_arg: bool,
        owner_type: Type,
        mock_type: &MockType,
        output_type_generics: OutputTypeGenerics,
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
            .generate_input_args(
                fn_info,
                fn_info
                    .parent
                    .get_internal_phantom_types_count_without_return_type()
                    + mock_type.generics.get_phantom_fields_count(),
            )
            .into_iter()
            .chain(iter::once(times_arg))
            .collect();
        if prepend_ref_self_arg {
            inputs.insert(0, constants::REF_SELF_ARG.clone());
        }
        let output_type =
            self.generate_output_type(fn_info.parent.arg_refs_tuple.clone(), owner_type);
        let generics = match output_type_generics {
            OutputTypeGenerics::UseFnOwn => fn_info.parent.own_generics.clone(),
            OutputTypeGenerics::UseMock => mock_type.generics.impl_generics.clone(),
            OutputTypeGenerics::DoNotUse => Default::default(),
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
            output: ReturnType::Type(Default::default(), Box::new(output_type)),
        };
        return signature;
    }

    fn generate_output_type(&self, mut arg_refs_tuple: Type, owner_type: Type) -> Type {
        self.reference_normalizer
            .normalize_anonymous_lifetimes(&mut arg_refs_tuple);
        let result = Type::Path(TypePath {
            qself: None,
            path: Path {
                leading_colon: None,
                segments: [PathSegment {
                    ident: constants::FN_VERIFIER_TYPE_IDENT.clone(),
                    arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                        colon2_token: None,
                        lt_token: Default::default(),
                        args: [
                            GenericArgument::Type(owner_type),
                            GenericArgument::Type(arg_refs_tuple),
                        ]
                        .into_iter()
                        .collect(),
                        gt_token: Default::default(),
                    }),
                }]
                .into_iter()
                .collect(),
            },
        });
        return result;
    }
}
