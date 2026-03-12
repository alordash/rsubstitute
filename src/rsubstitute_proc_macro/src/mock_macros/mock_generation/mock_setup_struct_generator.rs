use crate::constants;
use crate::mock_macros::mock_generation::models::*;
use crate::mock_macros::mock_generation::*;
use crate::syntax::*;
use proc_macro2::Ident;
use quote::format_ident;
use std::sync::Arc;
use syn::*;

pub(crate) trait IMockSetupStructGenerator {
    fn generate(
        &self,
        mock_ident: &Ident,
        mock_type: &MockType,
        mock_data_struct: &MockDataStruct,
        implemented_traits_configurators: Vec<ImplementedTraitConfigurator>,
    ) -> MockSetupStruct;
}

pub(crate) struct MockSetupStructGenerator {
    pub implemented_trait_ident_formatter: Arc<dyn IImplementedTraitIdentFormatter>,
}

impl IMockSetupStructGenerator for MockSetupStructGenerator {
    fn generate(
        &self,
        mock_ident: &Ident,
        mock_type: &MockType,
        mock_data_struct: &MockDataStruct,
        implemented_traits_configurators: Vec<ImplementedTraitConfigurator>,
    ) -> MockSetupStruct {
        let attrs = vec![
            constants::DOC_HIDDEN_ATTRIBUTE.clone(),
            constants::DERIVE_CLONE_FOR_RSUBSTITUTE_ATTRIBUTE.clone(),
        ];
        let ident = format_ident!("{}{}", mock_ident, Self::MOCK_SETUP_STRUCT_IDENT_SUFFIX);
        let data_type = mock_data_struct.ty.clone();
        let data_arc_type = r#type::wrap_in_arc(data_type);

        let fields = FieldsNamed {
            brace_token: Default::default(),
            named: [field::create(constants::DATA_IDENT.clone(), data_arc_type)]
                .into_iter()
                .chain(implemented_traits_configurators.into_iter().map(
                    |implemented_trait_setup| {
                        field::create_pub_from_struct(
                            self.implemented_trait_ident_formatter
                                .format_for_field(&implemented_trait_setup.trait_ident),
                            &implemented_trait_setup.item_struct,
                        )
                    },
                ))
                .collect(),
        };
        let item_struct = r#struct::create(
            attrs,
            ident,
            mock_type.generics.impl_generics.clone(),
            fields,
        );
        let ty = r#type::create_from_struct(&item_struct);
        let mock_setup_struct = MockSetupStruct { item_struct, ty };
        return mock_setup_struct;
    }
}

impl MockSetupStructGenerator {
    const MOCK_SETUP_STRUCT_IDENT_SUFFIX: &'static str = "Setup";
}
