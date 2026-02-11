use crate::mock_macros::fn_info_generation::models::*;
use crate::mock_macros::mock_generation::models::*;
use crate::mock_macros::mock_generation::*;
use crate::syntax::*;
use proc_macro2::Ident;
use std::sync::Arc;
use syn::*;

pub trait IMockPayloadImplGenerator {
    fn generate(
        &self,
        trait_ident: Ident,
        mock_type: &MockType,
        fn_infos: &[FnInfo],
    ) -> MockTraitImpl;

    fn generate_for_struct(
        &self,
        attrs: Vec<Attribute>,
        mock_type: &MockType,
        fn_infos: &[FnInfo],
    ) -> MockTraitImpl;
}

pub(crate) struct MockPayloadImplGenerator {
    pub path_factory: Arc<dyn IPathFactory>,
    pub mock_fn_block_generator: Arc<dyn IMockFnBlockGenerator>,
}

impl IMockPayloadImplGenerator for MockPayloadImplGenerator {
    fn generate(
        &self,
        trait_ident: Ident,
        mock_type: &MockType,
        fn_infos: &[FnInfo],
    ) -> MockTraitImpl {
        let trait_path = self
            .path_factory
            .create_with_generics(trait_ident, mock_type.generics.source_generics.clone());

        let mock_impl = self.generate_core(
            Vec::new(),
            mock_type.ty.clone(),
            mock_type.generics.impl_generics.clone(),
            fn_infos,
            Some(trait_path),
        );
        return mock_impl;
    }

    fn generate_for_struct(
        &self,
        attrs: Vec<Attribute>,
        mock_type: &MockType,
        fn_infos: &[FnInfo],
    ) -> MockTraitImpl {
        let mock_impl = self.generate_core(
            attrs,
            mock_type.ty.clone(),
            mock_type.generics.impl_generics.clone(),
            fn_infos,
            None,
        );
        return mock_impl;
    }
}

impl MockPayloadImplGenerator {
    fn generate_core(
        &self,
        attrs: Vec<Attribute>,
        self_ty: Type,
        generics: Generics,
        fn_infos: &[FnInfo],
        maybe_trait_path: Option<Path>,
    ) -> MockTraitImpl {
        let items = fn_infos
            .iter()
            .map(|x| self.generate_impl_item_fn(x))
            .map(ImplItem::Fn)
            .collect();
        let trait_ = maybe_trait_path.map(|trait_path| (None, trait_path, Default::default()));

        let item_impl = ItemImpl {
            attrs,
            defaultness: None,
            unsafety: None,
            impl_token: Default::default(),
            generics,
            trait_,
            self_ty: Box::new(self_ty),
            brace_token: Default::default(),
            items,
        };
        let mock_impl = MockTraitImpl { item_impl };
        return mock_impl;
    }

    fn generate_impl_item_fn(&self, fn_info: &FnInfo) -> ImplItemFn {
        let sig = Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: Default::default(),
            ident: fn_info.parent.fn_ident.clone(),
            generics: Generics::default(),
            paren_token: Default::default(),
            inputs: fn_info.parent.arguments.iter().cloned().collect(),
            variadic: None,
            output: fn_info.parent.return_value.clone(),
        };
        let block = self.mock_fn_block_generator.generate_for_trait(fn_info);
        let impl_item_fn = ImplItemFn {
            attrs: fn_info.parent.attrs.clone(),
            vis: fn_info.parent.visibility.clone(),
            defaultness: None,
            sig,
            block,
        };
        return impl_item_fn;
    }
}
