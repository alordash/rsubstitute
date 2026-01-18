use crate::constants;
use crate::mock_macros::fn_info_generation::IFnInfoGenerator;
use crate::mock_macros::mock_generation::models::*;
use crate::mock_macros::mock_generation::*;
use crate::mock_macros::*;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::sync::Arc;
use syn::*;

pub trait IItemTraitHandler {
    fn handle(&self, item_trait: ItemTrait) -> TokenStream;
}

pub(crate) struct ItemTraitHandler {
    pub fn_decl_extractor: Arc<dyn IFnDeclExtractor>,
    pub mock_generics_generator: Arc<dyn IMockGenericsGenerator>,
    pub fn_info_generator: Arc<dyn IFnInfoGenerator>,
    pub mock_data_struct_generator: Arc<dyn IMockDataStructGenerator>,
    pub mock_setup_struct_generator: Arc<dyn IMockSetupStructGenerator>,
    pub mock_received_struct_generator: Arc<dyn IMockReceivedStructGenerator>,
    pub mock_struct_generator: Arc<dyn IMockStructGenerator>,
    pub mock_trait_impl_generator: Arc<dyn IMockTraitImplGenerator>,
    pub mock_impl_generator: Arc<dyn IMockImplGenerator>,
    pub mock_setup_impl_generator: Arc<dyn IMockSetupImplGenerator>,
    pub mock_received_impl_generator: Arc<dyn IMockReceivedImplGenerator>,
    pub mod_generator: Arc<dyn IModGenerator>,
}

impl IItemTraitHandler for ItemTraitHandler {
    fn handle(&self, item_trait: ItemTrait) -> TokenStream {
        let mock_ident = format_ident!(
            "{}{}",
            item_trait.ident,
            constants::MOCK_STRUCT_IDENT_PREFIX
        );
        let fn_decls = self.fn_decl_extractor.extract(&item_trait.items);
        let target_ident = item_trait.ident.clone();
        let mock_generics = self.mock_generics_generator.generate(&item_trait.generics);
        let fn_infos: Vec<_> = fn_decls
            .iter()
            .map(|x| self.fn_info_generator.generate(x, &mock_generics))
            .collect();
        let mock_data_struct = self.mock_data_struct_generator.generate_for_trait(
            &mock_ident,
            &mock_generics,
            &fn_infos,
        );
        let mock_setup_struct = self.mock_setup_struct_generator.generate(
            &mock_ident,
            &mock_generics,
            &mock_data_struct,
        );
        let mock_received_struct = self.mock_received_struct_generator.generate(
            &mock_ident,
            &mock_generics,
            &mock_data_struct,
        );
        let mock_struct = self.mock_struct_generator.generate(
            mock_ident.clone(),
            &mock_generics,
            &mock_setup_struct,
            &mock_received_struct,
            &mock_data_struct,
        );
        let mock_trait_impl =
            self.mock_trait_impl_generator
                .generate(target_ident.clone(), &mock_struct, &fn_infos);
        let mock_impl = self.mock_impl_generator.generate(
            &mock_struct,
            &mock_data_struct,
            &mock_setup_struct,
            &mock_received_struct,
        );
        let mock_setup_impl = self
            .mock_setup_impl_generator
            .generate_for_trait(&mock_setup_struct, &fn_infos);
        let mock_received_impl = self
            .mock_received_impl_generator
            .generate_for_trait(&mock_received_struct, &fn_infos);
        let generated_mod = self.mod_generator.generate_trait(
            target_ident,
            fn_infos,
            mock_data_struct,
            mock_setup_struct,
            mock_received_struct,
            mock_struct,
            mock_trait_impl,
            mock_impl,
            mock_setup_impl,
            mock_received_impl,
        );

        let GeneratedMod {
            item_mod,
            use_generated_mod,
        } = generated_mod;
        let result = quote! {
            #item_trait

            #use_generated_mod
            #item_mod
        };
        return result.into();
    }
}
