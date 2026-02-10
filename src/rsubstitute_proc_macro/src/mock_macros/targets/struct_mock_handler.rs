use crate::constants;
use crate::mock_macros::fn_info_generation::IFnInfoGenerator;
use crate::mock_macros::mock_generation::models::*;
use crate::mock_macros::mock_generation::*;
use crate::mock_macros::models::*;
use crate::mock_macros::*;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::sync::Arc;

pub trait IStructMockHandler {
    fn handle(&self, struct_mock_syntax: StructMockSyntax) -> TokenStream;
}

pub struct StructMockHandler {
    pub mock_struct_trait_info_generator: Arc<dyn IMockStructTraitInfoGenerator>,
    pub fn_decl_extractor: Arc<dyn IFnDeclExtractor>,
    pub mock_generics_generator: Arc<dyn IMockGenericsGenerator>,
    pub mock_type_generator: Arc<dyn IMockTypeGenerator>,
    pub fn_info_generator: Arc<dyn IFnInfoGenerator>,
    pub mock_data_struct_generator: Arc<dyn IMockDataStructGenerator>,
    pub mock_setup_struct_generator: Arc<dyn IMockSetupStructGenerator>,
    pub mock_received_struct_generator: Arc<dyn IMockReceivedStructGenerator>,
    pub inner_data_struct_generator: Arc<dyn IInnerDataStructGenerator>,
    pub inner_data_impl_generator: Arc<dyn IInnerDataImplGenerator>,
    pub inner_data_param_generator: Arc<dyn IInnerDataParamGenerator>,
    pub mock_struct_generator: Arc<dyn IMockStructGenerator>,
    pub inner_data_deref_impl_generator: Arc<dyn IInnerDataDerefImplGenerator>,
    pub mock_struct_trait_generator: Arc<dyn IMockStructTraitGenerator>,
    pub mock_trait_impl_generator: Arc<dyn IMockTraitImplGenerator>,
    pub mock_impl_generator: Arc<dyn IMockImplGenerator>,
    pub mock_setup_impl_generator: Arc<dyn IMockSetupImplGenerator>,
    pub mock_received_impl_generator: Arc<dyn IMockReceivedImplGenerator>,
    pub ignored_impl_fixer: Arc<dyn IIgnoredImplFixer>,
    pub mod_generator: Arc<dyn IModGenerator>,
}

impl IStructMockHandler for StructMockHandler {
    fn handle(&self, mut struct_mock_syntax: StructMockSyntax) -> TokenStream {
        let source_struct_impls_syntax =
            self.generate_source_struct_impls_syntax(&struct_mock_syntax);

        let mock_ident = format_ident!(
            "{}{}",
            struct_mock_syntax.r#struct.ident,
            constants::MOCK_STRUCT_IDENT_PREFIX
        );

        let mock_generics = self
            .mock_generics_generator
            .generate(&struct_mock_syntax.r#struct.generics);
        let mock_type = self
            .mock_type_generator
            .generate_for_trait(mock_ident.clone(), mock_generics);
        let mock_struct_trait_infos: Vec<_> = std::mem::take(&mut struct_mock_syntax.trait_impls)
            .into_iter()
            .map(|x| {
                self.mock_struct_trait_info_generator
                    .generate(&mock_type, x)
            })
            .collect();
        let struct_fn_decls = self
            .fn_decl_extractor
            .extract_struct_fns(&struct_mock_syntax.get_struct_fns());
        let target_ident = struct_mock_syntax.r#struct.ident.clone();
        let struct_fn_infos: Vec<_> = struct_fn_decls
            .into_iter()
            .map(|x| self.fn_info_generator.generate(x, &mock_type))
            .collect();
        let all_fn_infos: Vec<_> = struct_fn_infos
            .iter()
            .chain(mock_struct_trait_infos.iter().flat_map(|x| &x.fn_infos))
            .collect();
        let mock_data_struct = self
            .mock_data_struct_generator
            .generate_for_trait(&mock_type, &all_fn_infos);
        let mock_struct_traits: Vec<_> = mock_struct_trait_infos
            .into_iter()
            .map(|mock_struct_trait_info| {
                self.mock_struct_trait_generator
                    .generate(&mock_data_struct, mock_struct_trait_info)
            })
            .collect();
        let mock_setup_struct = self.mock_setup_struct_generator.generate(
            &mock_ident,
            &mock_type,
            &mock_data_struct,
            mock_struct_traits
                .iter()
                .map(|mock_struct_trait| ImplementedTraitConfigurator {
                    trait_ident: mock_struct_trait.info.trait_ident_from_path.clone(),
                    item_struct: &mock_struct_trait.setup_struct.item_struct,
                })
                .collect(),
        );
        let mock_received_struct = self.mock_received_struct_generator.generate(
            &mock_ident,
            &mock_type,
            &mock_data_struct,
            mock_struct_traits
                .iter()
                .map(|mock_struct_trait| ImplementedTraitConfigurator {
                    trait_ident: mock_struct_trait.info.trait_ident_from_path.clone(),
                    item_struct: &mock_struct_trait.received_struct.item_struct,
                })
                .collect(),
        );
        let inner_data_struct = self
            .inner_data_struct_generator
            .generate(struct_mock_syntax.r#struct);
        let mock_struct = self.mock_struct_generator.generate(
            &mock_type,
            &mock_setup_struct,
            &mock_received_struct,
            &mock_data_struct,
            Some(&inner_data_struct),
        );
        let inner_data_deref_impl = self
            .inner_data_deref_impl_generator
            .generate(&mock_struct, &inner_data_struct);
        let mock_trait_impls = mock_struct_traits
            .iter()
            .map(|mock_struct_trait| {
                self.mock_trait_impl_generator.generate(
                    mock_struct_trait.info.trait_ident_from_path.clone(),
                    &mock_type,
                    &mock_struct_trait.info.fn_infos,
                )
            })
            .collect();
        let mock_trait_impl = self
            .mock_trait_impl_generator
            .generate_for_struct(&mock_type, &struct_fn_infos);
        let inner_data_param = self.inner_data_param_generator.generate(&inner_data_struct, &struct_mock_syntax.new_fn);
        let inner_data_impl = self
            .inner_data_impl_generator
            .generate(&inner_data_struct, struct_mock_syntax.new_fn);
        let mock_impl = self.mock_impl_generator.generate(
            &mock_type,
            &mock_struct,
            &mock_data_struct,
            &mock_setup_struct,
            &mock_received_struct,
            mock_struct_traits.iter().collect(),
            Some(inner_data_param),
        );
        let mock_setup_impl = self.mock_setup_impl_generator.generate_for_trait(
            &mock_type,
            &mock_setup_struct,
            &struct_fn_infos,
        );
        let mock_received_impl = self.mock_received_impl_generator.generate_for_trait(
            &mock_type,
            &mock_received_struct,
            &struct_fn_infos,
        );
        self.ignored_impl_fixer
            .fix(&mock_type, &mut struct_mock_syntax.ignored_impls);
        let generated_mod = self.mod_generator.generate_struct(
            target_ident,
            mock_struct_traits,
            struct_fn_infos,
            mock_data_struct,
            mock_setup_struct,
            mock_received_struct,
            inner_data_struct,
            inner_data_impl,
            mock_struct,
            inner_data_deref_impl,
            mock_trait_impls,
            mock_trait_impl,
            mock_impl,
            mock_setup_impl,
            mock_received_impl,
            struct_mock_syntax.ignored_impls,
        );

        let GeneratedMod {
            item_mod,
            use_generated_mod,
        } = generated_mod;
        let result = quote! {
            #source_struct_impls_syntax

            #use_generated_mod
            #item_mod
        };
        return result.into();
    }
}

impl StructMockHandler {
    fn generate_source_struct_impls_syntax(
        &self,
        struct_mock_syntax: &StructMockSyntax,
    ) -> proc_macro2::TokenStream {
        let cfg_not_test_attribute = constants::CFG_NOT_TEST_ATTRIBUTE.clone();
        let struct_syntax_var = &struct_mock_syntax.r#struct;
        let struct_syntax = quote! {
            #cfg_not_test_attribute
            #struct_syntax_var
        };

        let trait_impls_syntaxes: Vec<_> = struct_mock_syntax
            .trait_impls
            .iter()
            .map(|trait_impl| {
                let item_impl = &trait_impl.item_impl;
                quote! {
                    #cfg_not_test_attribute
                    #item_impl
                }
            })
            .collect();

        let struct_impls_syntaxes: Vec<_> = struct_mock_syntax
            .struct_impls
            .iter()
            .map(|struct_impl| {
                quote! {
                    #cfg_not_test_attribute
                    #struct_impl
                }
            })
            .collect();

        let result = quote! {
            #struct_syntax

            #(#trait_impls_syntaxes)*

            #(#struct_impls_syntaxes)*
        };
        return result;
    }
}
