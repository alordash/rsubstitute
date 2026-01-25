use crate::constants;
use crate::lifetime_ref::LifetimeRef;
use crate::mock_macros::fn_info_generation::models::*;
use crate::mock_macros::mock_generation::models::*;
use crate::mock_macros::mock_generation::*;
use crate::syntax::*;
use std::sync::Arc;
use syn::punctuated::Punctuated;
use syn::*;

pub trait IStaticFnGenerator {
    fn generate(
        &self,
        fn_info: &FnInfo,
        mock_struct: &MockStruct,
        mock_generics: &MockGenerics,
    ) -> StaticFn;
}

pub(crate) struct StaticFnGenerator {
    pub mock_fn_block_generator: Arc<dyn IMockFnBlockGenerator>,
    pub reference_normalizer: Arc<dyn IReferenceNormalizer>,
}

impl IStaticFnGenerator for StaticFnGenerator {
    fn generate(
        &self,
        fn_info: &FnInfo,
        mock_struct: &MockStruct,
        mock_generics: &MockGenerics,
    ) -> StaticFn {
        let mut generics = mock_generics.impl_generics.clone();
        generics.params.insert(
            0,
            GenericParam::Lifetime(LifetimeParam {
                attrs: Vec::new(),
                lifetime: constants::ANONYMOUS_LIFETIME.clone(),
                colon_token: Default::default(),
                bounds: Punctuated::new(),
            }),
        );
        let sig = Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: Default::default(),
            ident: fn_info.parent.ident.clone(),
            generics,
            paren_token: Default::default(),
            inputs: fn_info
                .parent
                .arguments
                .clone()
                .into_iter()
                .map(|mut fn_arg| {
                    self.reference_normalizer.anonymize_fn_arg(&mut fn_arg);
                    return fn_arg;
                })
                .collect(),
            variadic: None,
            output: fn_info.parent.return_value.clone(),
        };
        let block = self
            .mock_fn_block_generator
            .generate_for_static(fn_info, mock_struct);
        let item_fn = ItemFn {
            attrs: Vec::new(),
            vis: Visibility::Public(Default::default()),
            sig,
            block: Box::new(block),
        };
        let static_fn = StaticFn { item_fn };
        return static_fn;
    }
}
