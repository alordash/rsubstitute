use crate::constants;
use crate::lifetime_ref::LifetimeRef;
use crate::mock_macros::fn_info_generation::models::*;
use crate::mock_macros::mock_generation::models::*;
use crate::mock_macros::mock_generation::*;
use crate::syntax::*;
use proc_macro2::Ident;
use std::sync::Arc;
use syn::punctuated::Punctuated;
use syn::*;

pub trait IMockTraitImplGenerator {
    fn generate(
        &self,
        target_ident: Ident,
        mock_struct: &MockStruct,
        fn_infos: &[FnInfo],
    ) -> MockTraitImpl;
}

pub(crate) struct MockTraitImplGenerator {
    pub path_factory: Arc<dyn IPathFactory>,
    pub type_factory: Arc<dyn ITypeFactory>,
    pub reference_normalizer: Arc<dyn IReferenceNormalizer>,
    pub reference_type_crawler: Arc<dyn IReferenceTypeCrawler>,
    pub mock_fn_block_generator: Arc<dyn IMockFnBlockGenerator>,
}

impl IMockTraitImplGenerator for MockTraitImplGenerator {
    fn generate(
        &self,
        target_ident: Ident,
        mock_struct: &MockStruct,
        fn_infos: &[FnInfo],
    ) -> MockTraitImpl {
        let trait_ = self.path_factory.create(target_ident);
        let self_ty = self
            .type_factory
            .create(mock_struct.item_struct.ident.clone());
        let items = fn_infos
            .iter()
            .map(|x| self.generate_impl_item_fn(x))
            .map(ImplItem::Fn)
            .collect();

        let mut item_impl = ItemImpl {
            attrs: Vec::new(),
            defaultness: None,
            unsafety: None,
            impl_token: Default::default(),
            generics: constants::DEFAULT_ARG_FIELD_LIFETIME_GENERIC.clone(),
            trait_: Some((None, trait_, Default::default())),
            self_ty: Box::new(self_ty),
            brace_token: Default::default(),
            items,
        };
        self.reference_normalizer.normalize_in_impl(
            constants::DEFAULT_ARG_FIELD_LIFETIME.clone(),
            &mut item_impl,
        );
        let mock_impl = MockTraitImpl { item_impl };
        return mock_impl;
    }
}

impl MockTraitImplGenerator {
    fn generate_impl_item_fn(&self, fn_info: &FnInfo) -> ImplItemFn {
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
        let block = self.mock_fn_block_generator.generate_for_trait(fn_info);
        let impl_item_fn = ImplItemFn {
            attrs: Vec::new(),
            vis: Visibility::Inherited,
            defaultness: None,
            sig,
            block,
        };
        return impl_item_fn;
    }

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
