use crate::constants;
use crate::lifetime_ref::LifetimeRef;
use crate::mock_macros::fn_info_generation::models::*;
use crate::mock_macros::mock_generation::models::*;
use crate::mock_macros::mock_generation::*;
use crate::syntax::IReferenceTypeCrawler;
use std::sync::Arc;
use syn::punctuated::Punctuated;
use syn::*;

pub trait IStaticFnGenerator {
    fn generate(&self, fn_info: &FnInfo, static_mock: &StaticMock) -> StaticFn;
}

pub(crate) struct StaticFnGenerator {
    pub mock_fn_block_generator: Arc<dyn IMockFnBlockGenerator>,
    pub reference_type_crawler: Arc<dyn IReferenceTypeCrawler>,
}

impl IStaticFnGenerator for StaticFnGenerator {
    fn generate(&self, fn_info: &FnInfo, static_mock: &StaticMock) -> StaticFn {
        let sig = Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: Default::default(),
            ident: fn_info.parent.ident.clone(),
            generics: Generics {
                lt_token: Some(Default::default()),
                params: [GenericParam::Lifetime(LifetimeParam {
                    attrs: Vec::new(),
                    lifetime: constants::ANONYMOUS_LIFETIME.clone(),
                    colon_token: None,
                    bounds: Punctuated::new(),
                })]
                .into_iter()
                .collect(),
                gt_token: Some(Default::default()),
                where_clause: None,
            },
            paren_token: Default::default(),
            inputs: fn_info
                .parent
                .arguments
                .clone()
                .into_iter()
                .map(|mut fn_arg| {
                    self.convert_input_reference(&mut fn_arg);
                    return fn_arg;
                })
                .collect(),
            variadic: None,
            output: fn_info.parent.return_value.clone(),
        };
        let block = self
            .mock_fn_block_generator
            .generate_for_static(fn_info, static_mock);
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

impl StaticFnGenerator {
    // TODO - remove duplicates
    fn convert_input_reference(&self, fn_arg: &mut FnArg) {
        let ty = match fn_arg {
            FnArg::Receiver(receiver) => {
                if let Some((_, lifetime)) = &mut receiver.reference {
                    self.anonymize_input_reference_lifetime(LifetimeRef::Optional(lifetime));
                }
                receiver.ty.as_mut()
            }
            FnArg::Typed(pat_type) => pat_type.ty.as_mut(),
        };
        let lifetime_refs = self.reference_type_crawler.get_all_type_references(ty);
        for lifetime_ref in lifetime_refs {
            self.anonymize_input_reference_lifetime(lifetime_ref);
        }
    }

    fn anonymize_input_reference_lifetime(&self, lifetime_ref: LifetimeRef) {
        if let LifetimeRef::Optional(optional_lifetime) = lifetime_ref
            && optional_lifetime.is_none()
        {
            *optional_lifetime = Some(constants::ANONYMOUS_LIFETIME.clone());
        }
    }
}
