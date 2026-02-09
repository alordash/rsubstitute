use crate::constants;
use crate::mock_macros::fn_info_generation::models::FnInfo;
use crate::mock_macros::mock_generation::models::*;
use crate::syntax::*;
use quote::format_ident;
use std::sync::Arc;
use syn::*;

pub trait IMockDataStructGenerator {
    fn generate_for_trait(&self, mock_type: &MockType, all_fn_infos: &[&FnInfo]) -> MockDataStruct;

    fn generate_for_static(&self, mock_type: &MockType, all_fn_infos: &[&FnInfo]) -> MockDataStruct;
}

// TODO - verify all impls are internal
pub(crate) struct MockDataStructGenerator {
    pub type_factory: Arc<dyn ITypeFactory>,
    pub field_factory: Arc<dyn IFieldFactory>,
    pub struct_factory: Arc<dyn IStructFactory>,
}

impl IMockDataStructGenerator for MockDataStructGenerator {
    fn generate_for_trait(&self, mock_type: &MockType, fn_infos: &[&FnInfo]) -> MockDataStruct {
        let attrs = vec![constants::DERIVE_MOCK_DATA_ATTRIBUTE.clone()];
        let ident = format_ident!(
            "{}{}",
            mock_type.ident.clone(),
            Self::MOCK_DATA_STRUCT_IDENT_SUFFIX
        );
        let fn_fields: Vec<_> = fn_infos
            .iter()
            .map(|x| self.generate_field(x, mock_type))
            .collect();
        let field_and_fn_idents = fn_fields
            .iter()
            .zip(fn_infos)
            .map(|(x, y)| (x.get_required_ident(), y.parent.ident.clone()))
            .collect();
        let fields = std::iter::once(constants::DEFAULT_ARG_FIELD_LIFETIME_FIELD.clone())
            .chain(fn_fields)
            .collect();
        let fields_named = FieldsNamed {
            brace_token: Default::default(),
            named: fields,
        };

        let item_struct = self.struct_factory.create(
            attrs,
            ident,
            mock_type.generics.impl_generics.clone(),
            fields_named,
        );
        let mock_struct = MockDataStruct {
            item_struct,
            field_and_fn_idents,
        };
        return mock_struct;
    }

    fn generate_for_static(&self, mock_type: &MockType, fn_infos: &[&FnInfo]) -> MockDataStruct {
        let attrs = vec![
            constants::ALLOW_NON_CAMEL_CASE_TYPES_ATTRIBUTE.clone(),
            constants::DERIVE_MOCK_DATA_ATTRIBUTE.clone(),
        ];
        let ident = format_ident!(
            "{}{}",
            mock_type.ident.clone(),
            Self::MOCK_DATA_STRUCT_IDENT_SUFFIX
        );
        let fn_fields: Vec<_> = fn_infos
            .iter()
            .map(|x| self.generate_field(x, mock_type))
            .collect();
        let field_and_fn_idents = fn_fields
            .iter()
            .zip(fn_infos)
            .map(|(x, y)| (x.get_required_ident(), y.parent.ident.clone()))
            .collect();
        let fields = [constants::DEFAULT_ARG_FIELD_LIFETIME_FIELD.clone()]
            .into_iter()
            .chain(fn_fields)
            .collect();
        let fields_named = FieldsNamed {
            brace_token: Default::default(),
            named: fields,
        };

        let item_struct = self.struct_factory.create(
            attrs,
            ident,
            mock_type.generics.impl_generics.clone(),
            fields_named,
        );
        let mock_struct = MockDataStruct {
            item_struct,
            field_and_fn_idents,
        };
        return mock_struct;
    }
}

impl MockDataStructGenerator {
    const MOCK_DATA_STRUCT_IDENT_SUFFIX: &'static str = "Data";

    fn generate_field(&self, fn_info: &FnInfo, mock_type: &MockType) -> Field {
        let ty = Type::Path(TypePath {
            qself: None,
            path: Path {
                leading_colon: None,
                segments: [PathSegment {
                    ident: constants::FN_DATA_TYPE_IDENT.clone(),
                    arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                        colon2_token: None,
                        lt_token: Default::default(),
                        args: [
                            GenericArgument::Type(mock_type.ty.clone()),
                            GenericArgument::Type(
                                self.type_factory
                                    .create_from_struct(&fn_info.call_struct.item_struct),
                            ),
                            GenericArgument::Type(
                                self.type_factory
                                    .create_from_struct(&fn_info.args_checker_struct.item_struct),
                            ),
                            GenericArgument::Type(fn_info.parent.get_return_value_type()),
                        ]
                        .into_iter()
                        .collect(),
                        gt_token: Default::default(),
                    }),
                }]
                .into_iter()
                .collect(),
            },
        });
        let field = self
            .field_factory
            .create(fn_info.data_field_ident.clone(), ty);
        return field;
    }
}
