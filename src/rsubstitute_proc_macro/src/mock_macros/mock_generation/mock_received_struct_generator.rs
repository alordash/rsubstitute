use crate::constants;
use crate::mock_macros::mock_generation::models::*;
use crate::syntax::*;
use proc_macro2::Ident;
use quote::format_ident;
use std::sync::Arc;
use syn::*;

pub trait IMockReceivedStructGenerator {
    fn generate(&self, mock_ident: &Ident, mock_data_struct: &MockDataStruct)
    -> MockReceivedStruct;
    fn generate_with_non_camel_case_allowed(
        &self,
        mock_ident: &Ident,
        mock_data_struct: &MockDataStruct,
    ) -> MockReceivedStruct;
}

pub(crate) struct MockReceivedStructGenerator {
    pub type_factory: Arc<dyn ITypeFactory>,
    pub field_factory: Arc<dyn IFieldFactory>,
    pub struct_factory: Arc<dyn IStructFactory>,
}

impl IMockReceivedStructGenerator for MockReceivedStructGenerator {
    fn generate(
        &self,
        mock_ident: &Ident,
        mock_data_struct: &MockDataStruct,
    ) -> MockReceivedStruct {
        let attrs = Vec::new();
        let ident = format_ident!("{}{}", mock_ident, Self::MOCK_RECEIVED_STRUCT_IDENT_SUFFIX);
        let data_type = self
            .type_factory
            .create_from_struct(&mock_data_struct.item_struct);
        let data_arc_type = self.type_factory.wrap_in_arc(data_type);
        let fields = FieldsNamed {
            brace_token: Default::default(),
            named: [self
                .field_factory
                .create(constants::DATA_IDENT.clone(), data_arc_type)]
            .into_iter()
            .collect(),
        };
        let item_struct = self.struct_factory.create(attrs, ident, fields);
        let mock_received_struct = MockReceivedStruct { item_struct };
        return mock_received_struct;
    }
    fn generate_with_non_camel_case_allowed(
        &self,
        mock_ident: &Ident,
        mock_data_struct: &MockDataStruct,
    ) -> MockReceivedStruct {
        let mut result = self.generate(mock_ident, mock_data_struct);
        result
            .item_struct
            .attrs
            .push(constants::ALLOW_NON_CAMEL_CASE_TYPES_ATTRIBUTE.clone());
        return result;
    }
}

impl MockReceivedStructGenerator {
    const MOCK_RECEIVED_STRUCT_IDENT_SUFFIX: &'static str = "Received";
}
