use crate::constants;
use crate::mock_macros::mock_generation::models::*;
use crate::syntax::*;
use std::sync::Arc;
use syn::*;

pub trait IMockStructGenerator {
    fn generate(
        &self,
        mock_type: &MockType,
        mock_setup_struct: &MockSetupStruct,
        mock_received_struct: &MockReceivedStruct,
        mock_data_struct: &MockDataStruct,
    ) -> MockStruct;

    fn generate_for_static(
        &self,
        mock_type: &MockType,
        mock_setup_struct: &MockSetupStruct,
        mock_received_struct: &MockReceivedStruct,
        mock_data_struct: &MockDataStruct,
    ) -> MockStruct;
}

pub(crate) struct MockStructGenerator {
    pub field_factory: Arc<dyn IFieldFactory>,
    pub type_factory: Arc<dyn ITypeFactory>,
    pub struct_factory: Arc<dyn IStructFactory>,
    pub reference_normalizer: Arc<dyn IReferenceNormalizer>,
}

impl IMockStructGenerator for MockStructGenerator {
    fn generate(
        &self,
        mock_type: &MockType,
        mock_setup_struct: &MockSetupStruct,
        mock_received_struct: &MockReceivedStruct,
        mock_data_struct: &MockDataStruct,
    ) -> MockStruct {
        let attrs = vec![constants::ALLOW_NON_CAMEL_CASE_TYPES_ATTRIBUTE.clone()];
        let data_field = self.field_factory.create(
            constants::DATA_IDENT.clone(),
            self.type_factory.wrap_in_arc(
                self.type_factory
                    .create_from_struct(&mock_data_struct.item_struct),
            ),
        );
        let fields = FieldsNamed {
            brace_token: Default::default(),
            named: [
                self.field_factory.create_pub_from_struct(
                    constants::MOCK_SETUP_FIELD_IDENT.clone(),
                    &mock_setup_struct.item_struct,
                ),
                self.field_factory.create_pub_from_struct(
                    constants::MOCK_RECEIVED_FIELD_IDENT.clone(),
                    &mock_received_struct.item_struct,
                ),
                data_field,
            ]
            .into_iter()
            .collect(),
        };
        let item_struct = self.struct_factory.create(
            attrs,
            mock_type.ident.clone(),
            mock_type.generics.clone(),
            fields,
        );
        let result = MockStruct { item_struct };
        return result;
    }

    fn generate_for_static(
        &self,
        mock_type: &MockType,
        mock_setup_struct: &MockSetupStruct,
        mock_received_struct: &MockReceivedStruct,
        mock_data_struct: &MockDataStruct,
    ) -> MockStruct {
        let mut mock_struct = self.generate(
            mock_type,
            mock_setup_struct,
            mock_received_struct,
            mock_data_struct,
        );
        mock_struct.item_struct.generics.params = mock_struct
            .item_struct
            .generics
            .params
            .into_iter()
            .skip(1)
            .collect();
        for field in mock_struct.item_struct.fields.iter_mut() {
            self.reference_normalizer.staticify(&mut field.ty);
        }

        return mock_struct;
    }
}
