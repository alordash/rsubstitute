use crate::constants;
use crate::mock_macros::mock_generation::models::*;
use crate::syntax::*;
use syn::*;

pub(crate) trait IMockStructGenerator {
    fn generate(
        &self,
        attrs: Vec<Attribute>,
        mock_type: &MockType,
        mock_setup_struct: &MockSetupStruct,
        mock_received_struct: &MockReceivedStruct,
        mock_data_struct: &MockDataStruct,
        maybe_inner_data_struct: Option<&InnerDataStruct>,
    ) -> MockStruct;

    fn generate_for_static(
        &self,
        mock_type: &MockType,
        mock_setup_struct: &MockSetupStruct,
        mock_received_struct: &MockReceivedStruct,
        mock_data_struct: &MockDataStruct,
    ) -> MockStruct;
}

pub(crate) struct MockStructGenerator;

impl IMockStructGenerator for MockStructGenerator {
    fn generate(
        &self,
        attrs: Vec<Attribute>,
        mock_type: &MockType,
        mock_setup_struct: &MockSetupStruct,
        mock_received_struct: &MockReceivedStruct,
        mock_data_struct: &MockDataStruct,
        maybe_inner_data_struct: Option<&InnerDataStruct>,
    ) -> MockStruct {
        let data_field = field::create(
            constants::DATA_IDENT.clone(),
            r#type::wrap_in_arc(mock_data_struct.ty.clone()),
        );
        let fields = FieldsNamed {
            brace_token: Default::default(),
            named: [
                field::create_pub_from_struct(
                    constants::MOCK_SETUP_FIELD_IDENT.clone(),
                    &mock_setup_struct.item_struct,
                ),
                field::create_pub_from_struct(
                    constants::MOCK_RECEIVED_FIELD_IDENT.clone(),
                    &mock_received_struct.item_struct,
                ),
                data_field,
            ]
            .into_iter()
            .chain(
                maybe_inner_data_struct
                    .map(|inner_data_struct| {
                        field::create_from_struct(
                            constants::INNER_DATA_FIELD_IDENT.clone(),
                            &inner_data_struct.item_struct,
                        )
                    })
                    .into_iter(),
            )
            .collect(),
        };
        let item_struct = r#struct::create(
            attrs,
            mock_type.ident.clone(),
            mock_type.generics.impl_generics.clone(),
            fields,
        );
        let ty = r#type::create_from_struct(&item_struct);
        let result = MockStruct { item_struct, ty };
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
            Vec::new(),
            mock_type,
            mock_setup_struct,
            mock_received_struct,
            mock_data_struct,
            None,
        );
        mock_struct
            .item_struct
            .attrs
            .insert(0, constants::DOC_HIDDEN_ATTRIBUTE.clone());
        for field in mock_struct.item_struct.fields.iter_mut() {
            reference::staticify_anonymous_lifetimes(&mut field.ty);
        }

        return mock_struct;
    }
}
