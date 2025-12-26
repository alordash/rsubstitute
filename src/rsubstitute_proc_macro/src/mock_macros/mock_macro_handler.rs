use crate::mock_macros::fn_decl_extractor::IFnDeclExtractor;
use crate::mock_macros::fn_info_generation::IFnInfoGenerator;
use crate::mock_macros::mock_generation::{
    IInternalMockImplGenerator, IMockImplGenerator, IMockStructGenerator,
};
use crate::mock_macros::{IModGenerator, ITargetDeclExtractor};
use proc_macro::TokenStream;
use quote::quote;
use std::rc::Rc;
use syn::*;

pub trait IMockMacroHandler {
    fn handle(
        &self,
        proc_macro_attribute: proc_macro::TokenStream,
        proc_macro_item: proc_macro::TokenStream,
    ) -> proc_macro::TokenStream;
}

pub struct MockMacroHandler {
    pub(crate) target_decl_extractor: Rc<dyn ITargetDeclExtractor>,
    pub(crate) fn_decl_extractor: Rc<dyn IFnDeclExtractor>,
    pub(crate) fn_info_generator: Rc<dyn IFnInfoGenerator>,
    pub(crate) mock_struct_generator: Rc<dyn IMockStructGenerator>,
    pub(crate) mock_impl_generator: Rc<dyn IMockImplGenerator>,
    pub(crate) internal_mock_impl_generator: Rc<dyn IInternalMockImplGenerator>,
    pub(crate) mod_generator: Rc<dyn IModGenerator>,
}

impl IMockMacroHandler for MockMacroHandler {
    fn handle(
        &self,
        _proc_macro_attribute: TokenStream,
        proc_macro_item: TokenStream,
    ) -> TokenStream {
        if let Ok(item_impl) = syn::parse::<ItemImpl>(proc_macro_item.clone()) {
            return self.handle_item_impl(item_impl);
        } else if let Ok(item_trait) = syn::parse::<ItemTrait>(proc_macro_item) {
            return self.handle_item_trait(item_trait);
        }

        panic!("Expected `impl` or `trait`.");
    }
}

impl MockMacroHandler {
    fn handle_item_impl(&self, _item_impl: ItemImpl) -> TokenStream {
        todo!();
    }

    fn handle_item_trait(&self, item_trait: ItemTrait) -> TokenStream {
        let target_decl = self.target_decl_extractor.extract(&item_trait);
        let fn_decls = self.fn_decl_extractor.extract(&item_trait.items);
        let trait_ident = item_trait.ident.clone();
        let fn_infos: Vec<_> = fn_decls
            .iter()
            .map(|x| self.fn_info_generator.generate(x))
            .collect();
        let mock_struct_info = self.mock_struct_generator.generate(&target_decl, &fn_infos);
        let mock_impl_info =
            self.mock_impl_generator
                .generate(&target_decl, &mock_struct_info, &fn_infos);
        let internal_mock_impl_info = self
            .internal_mock_impl_generator
            .generate(&mock_struct_info, &fn_infos);
        let mod_info = self.mod_generator.generate(
            trait_ident,
            fn_infos,
            mock_struct_info,
            mock_impl_info,
            internal_mock_impl_info,
        );

        let use_generated_mod = ItemUse {
            attrs: Vec::new(),
            vis: Visibility::Public(Default::default()),
            use_token: Default::default(),
            leading_colon: None,
            tree: UseTree::Path(UsePath {
                ident: mod_info.item_mod.ident.clone(),
                colon2_token: Default::default(),
                tree: Box::new(UseTree::Glob(UseGlob {
                    star_token: Default::default(),
                })),
            }),
            semi_token: Default::default()
        };
        let generated_mod = mod_info.item_mod;
        let result = quote! {
            #item_trait

            #use_generated_mod
            #generated_mod
        };
        return result.into();
    }
}
