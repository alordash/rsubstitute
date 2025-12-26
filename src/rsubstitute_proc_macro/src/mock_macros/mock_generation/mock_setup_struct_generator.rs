use crate::constants;
use crate::mock_macros::mock_generation::models::*;
use crate::syntax::*;
use proc_macro2::Ident;
use quote::format_ident;
use std::rc::Rc;
use syn::*;

pub trait IMockSetupStructGenerator {
    fn generate(&self, mock_ident: &Ident, mock_data_struct: &MockDataStruct) -> MockSetupStruct;
}

pub(crate) struct MockSetupStructGenerator {
    pub type_factory: Rc<dyn ITypeFactory>,
    pub field_factory: Rc<dyn IFieldFactory>,
    pub struct_factory: Rc<dyn IStructFactory>,
}

impl IMockSetupStructGenerator for MockSetupStructGenerator {
    fn generate(&self, mock_ident: &Ident, mock_data_struct: &MockDataStruct) -> MockSetupStruct {
        let attrs = Vec::new();
        let ident = format_ident!("{}{}", mock_ident, Self::MOCK_SETUP_STRUCT_IDENT_SUFFIX);
        let data_type = self
            .type_factory
            .create_from_struct(&mock_data_struct.item_struct);
        let data_rc_type = self.type_factory.wrap_in_rc(data_type);
        let fields = FieldsNamed {
            brace_token: Default::default(),
            named: [self
                .field_factory
                .create(constants::DATA_IDENT.clone(), data_rc_type)]
            .into_iter()
            .collect(),
        };
        let item_struct = self
            .struct_factory
            .create(attrs, ident, fields);
        let mock_setup_struct = MockSetupStruct { item_struct };
        return mock_setup_struct;
    }
}

impl MockSetupStructGenerator {
    const MOCK_SETUP_STRUCT_IDENT_SUFFIX: &'static str = "Setup";
}
