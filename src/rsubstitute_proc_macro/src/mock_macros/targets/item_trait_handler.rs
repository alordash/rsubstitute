use crate::constants;
use crate::mock_macros::fn_info_generation::IFnInfoGenerator;
use crate::mock_macros::mock_generation::models::*;
use crate::mock_macros::mock_generation::*;
use crate::mock_macros::models::*;
use crate::mock_macros::*;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::sync::Arc;
use syn::*;

pub trait IItemTraitHandler {
    fn handle(&self, ctx: &Ctx, item_trait: ItemTrait) -> TokenStream;
}

pub(crate) struct ItemTraitHandler {
    pub fn_decl_extractor: Arc<dyn IFnDeclExtractor>,
    pub mock_generics_generator: Arc<dyn IMockGenericsGenerator>,
    pub mock_type_generator: Arc<dyn IMockTypeGenerator>,
    pub fn_info_generator: Arc<dyn IFnInfoGenerator>,
    pub mock_data_struct_generator: Arc<dyn IMockDataStructGenerator>,
    pub mock_setup_struct_generator: Arc<dyn IMockSetupStructGenerator>,
    pub mock_received_struct_generator: Arc<dyn IMockReceivedStructGenerator>,
    pub mock_struct_generator: Arc<dyn IMockStructGenerator>,
    pub mock_payload_impl_generator: Arc<dyn IMockPayloadImplGenerator>,
    pub mock_impl_generator: Arc<dyn IMockImplGenerator>,
    pub mock_setup_impl_generator: Arc<dyn IMockSetupImplGenerator>,
    pub mock_received_impl_generator: Arc<dyn IMockReceivedImplGenerator>,
    pub base_fn_generator: Arc<dyn IBaseFnGenerator>,
    pub mod_generator: Arc<dyn IModGenerator>,
}

impl IItemTraitHandler for ItemTraitHandler {
    fn handle(&self, ctx: &Ctx, item_trait: ItemTrait) -> TokenStream {
        let mock_ident = format_ident!(
            "{}{}",
            item_trait.ident,
            constants::MOCK_STRUCT_IDENT_PREFIX
        );
        let mock_generics = self
            .mock_generics_generator
            .generate(&item_trait.generics);
        let fn_decls = self
            .fn_decl_extractor
            .extract(ctx, &mock_generics, &item_trait.items);
        let target_ident = item_trait.ident.clone();
        let mock_type = self
            .mock_type_generator
            .generate(mock_ident.clone(), mock_generics);
        let fn_infos: Vec<_> = fn_decls
            .into_iter()
            .map(|x| self.fn_info_generator.generate(ctx, x, &mock_type))
            .collect();
        let all_fn_infos: Vec<_> = fn_infos.iter().collect();
        let mock_data_struct = self
            .mock_data_struct_generator
            .generate_for_trait(&mock_type, &all_fn_infos);
        let mock_setup_struct = self.mock_setup_struct_generator.generate(
            &mock_ident,
            &mock_type,
            &mock_data_struct,
            Vec::new(),
        );
        let mock_received_struct = self.mock_received_struct_generator.generate(
            &mock_ident,
            &mock_type,
            &mock_data_struct,
            Vec::new(),
        );
        let mock_struct = self.mock_struct_generator.generate(
            vec![constants::DERIVE_CLONE_FOR_RSUBSTITUTE_ATTRIBUTE.clone()],
            &mock_type,
            &mock_setup_struct,
            &mock_received_struct,
            &mock_data_struct,
            None,
        );
        let mock_trait_impl =
            self.mock_payload_impl_generator
                .generate(target_ident.clone(), &mock_type, &fn_infos);
        let base_fns: Vec<_> = fn_infos
            .iter()
            .filter_map(|fn_info| {
                fn_info
                    .parent
                    .maybe_base_fn_block
                    .clone()
                    .map(|base_fn_block| {
                        self.base_fn_generator.generate(
                            &mock_type,
                            &fn_info.parent,
                            &fn_info.call_struct,
                            base_fn_block,
                        )
                    })
            })
            .map(|base_fn| ImplItem::Fn(base_fn.impl_item_fn))
            .collect();
        let mock_impl = self.mock_impl_generator.generate(
            &mock_type,
            &mock_struct,
            &mock_data_struct,
            &mock_setup_struct,
            &mock_received_struct,
            Vec::new(),
            None,
            base_fns,
        );
        let mock_setup_impl = self.mock_setup_impl_generator.generate_for_trait(
            &mock_type,
            &mock_setup_struct,
            &fn_infos,
        );
        let mock_received_impl = self.mock_received_impl_generator.generate_for_trait(
            &mock_type,
            &mock_received_struct,
            &fn_infos,
        );
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
