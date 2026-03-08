use crate::constants;
use crate::mock_macros::mock_generation::models::*;
use crate::syntax::*;
use proc_macro2::Ident;
use quote::format_ident;
use std::sync::Arc;
use syn::*;

pub trait IMockSetupStructGenerator {
    fn generate(
        &self,
        mock_ident: &Ident,
        mock_type: &MockType,
        mock_data_struct: &MockDataStruct,
        implemented_traits_configurators: Vec<ImplementedTraitConfigurator>,
    ) -> MockSetupStruct;
}

pub(crate) struct MockSetupStructGenerator {
    pub type_factory: Arc<dyn ITypeFactory>,
    pub field_factory: Arc<dyn IFieldFactory>,
    pub struct_factory: Arc<dyn IStructFactory>,
}

impl IMockSetupStructGenerator for MockSetupStructGenerator {
    fn generate(
        &self,
        mock_ident: &Ident,
        mock_type: &MockType,
        mock_data_struct: &MockDataStruct,
        implemented_traits_configurators: Vec<ImplementedTraitConfigurator>,
    ) -> MockSetupStruct {
        let attrs = vec![constants::DOC_HIDDEN_ATTRIBUTE.clone()];
        let ident = format_ident!("{}{}", mock_ident, Self::MOCK_SETUP_STRUCT_IDENT_SUFFIX);
        let data_type = self
            .type_factory
            .create_from_struct(&mock_data_struct.item_struct);
        let data_arc_type = self.type_factory.wrap_in_arc(data_type);

        let fields =
            FieldsNamed {
                brace_token: Default::default(),
                named: [self
                    .field_factory
                    .create(constants::DATA_IDENT.clone(), data_arc_type)]
                .into_iter()
                .chain(implemented_traits_configurators.into_iter().map(
                    |implemented_trait_setup| {
                        self.field_factory.create_pub_from_struct(
                            implemented_trait_setup.trait_ident,
                            &implemented_trait_setup.item_struct,
                        )
                    },
                ))
                .collect(),
            };
        let item_struct = self.struct_factory.create(
            attrs,
            ident,
            mock_type.generics.impl_generics.clone(),
            fields,
        );
        let mock_setup_struct = MockSetupStruct { item_struct };
        return mock_setup_struct;
    }
}

impl MockSetupStructGenerator {
    const MOCK_SETUP_STRUCT_IDENT_SUFFIX: &'static str = "Setup";
}
