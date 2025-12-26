use crate::constants;
use crate::mock_macros::IModGenerator;
use crate::mock_macros::fn_decl_extractor::IFnDeclExtractor;
use crate::mock_macros::fn_info_generation::IFnInfoGenerator;
use crate::mock_macros::mock_generation::*;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::rc::Rc;
use syn::*;

pub trait IMockMacroHandler {
    fn handle(
        &self,
        proc_macro_attribute: proc_macro::TokenStream,
        proc_macro_item: proc_macro::TokenStream,
    ) -> proc_macro::TokenStream;
}

pub(crate) struct MockMacroHandler {
    pub fn_decl_extractor: Rc<dyn IFnDeclExtractor>,
    pub fn_info_generator: Rc<dyn IFnInfoGenerator>,
    pub mock_data_struct_generator: Rc<dyn IMockDataStructGenerator>,
    pub mock_setup_struct_generator: Rc<dyn IMockSetupStructGenerator>,
    pub mock_received_struct_generator: Rc<dyn IMockReceivedStructGenerator>,
    pub mock_struct_generator: Rc<dyn IMockStructGenerator>,
    pub mock_impl_generator: Rc<dyn IMockImplGenerator>,
    pub internal_mock_impl_generator: Rc<dyn IInternalMockImplGenerator>,
    pub internal_mock_setup_impl_generator: Rc<dyn IInternalMockSetupImplGenerator>,
    pub internal_mock_received_impl_generator: Rc<dyn IInternalMockReceivedImplGenerator>,
    pub mod_generator: Rc<dyn IModGenerator>,
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
        let mock_ident = format_ident!(
            "{}{}",
            item_trait.ident,
            constants::MOCK_STRUCT_IDENT_PREFIX
        );
        let fn_decls = self.fn_decl_extractor.extract(&item_trait.items);
        let target_ident = item_trait.ident.clone();
        let fn_infos: Vec<_> = fn_decls
            .iter()
            .map(|x| self.fn_info_generator.generate(x))
            .collect();
        let mock_data_struct = self
            .mock_data_struct_generator
            .generate(&mock_ident, &fn_infos);
        let mock_setup_struct = self
            .mock_setup_struct_generator
            .generate(&mock_ident, &mock_data_struct);
        let mock_received_struct = self
            .mock_received_struct_generator
            .generate(&mock_ident, &mock_data_struct);
        let mock_struct = self.mock_struct_generator.generate(
            mock_ident.clone(),
            &mock_setup_struct,
            &mock_received_struct,
            &mock_data_struct,
        );
        let mock_impl =
            self.mock_impl_generator
                .generate(target_ident.clone(), &mock_struct, &fn_infos);
        let internal_mock_impl = self.internal_mock_impl_generator.generate(
            &mock_struct,
            &mock_data_struct,
            &mock_setup_struct,
            &mock_received_struct,
        );
        let internal_mock_setup_impl = self
            .internal_mock_setup_impl_generator
            .generate(&mock_setup_struct, &fn_infos);
        let internal_mock_received_impl = self
            .internal_mock_received_impl_generator
            .generate(&mock_received_struct, &fn_infos);
        let mod_info = self.mod_generator.generate(
            target_ident,
            fn_infos,
            mock_data_struct,
            mock_setup_struct,
            mock_received_struct,
            mock_struct,
            mock_impl,
            internal_mock_impl,
            internal_mock_setup_impl,
            internal_mock_received_impl,
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
            semi_token: Default::default(),
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
