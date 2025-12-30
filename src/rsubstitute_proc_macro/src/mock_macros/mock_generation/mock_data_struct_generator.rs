use crate::constants;
use crate::mock_macros::fn_info_generation::models::FnInfo;
use crate::mock_macros::mock_generation::models::MockDataStruct;
use crate::syntax::{IStructFactory, ITypeFactory};
use quote::format_ident;
use std::sync::Arc;
use syn::*;

pub trait IMockDataStructGenerator {
    fn generate(&self, mock_ident: &Ident, fn_infos: &[FnInfo]) -> MockDataStruct;

    fn generate_with_non_camel_case_allowed(
        &self,
        mock_ident: &Ident,
        fn_infos: &[FnInfo],
    ) -> MockDataStruct;
}

// TODO - verify all impls are internal
pub(crate) struct MockDataStructGenerator {
    pub type_factory: Arc<dyn ITypeFactory>,
    pub struct_factory: Arc<dyn IStructFactory>,
}

impl IMockDataStructGenerator for MockDataStructGenerator {
    fn generate(&self, mock_ident: &Ident, fn_infos: &[FnInfo]) -> MockDataStruct {
        let attrs = Vec::new();
        let ident = format_ident!("{}{}", mock_ident, Self::MOCK_DATA_STRUCT_IDENT_SUFFIX);
        let fields = std::iter::once(constants::DEFAULT_ARG_FIELD_LIFETIME_FIELD.clone())
            .chain(fn_infos.iter().map(|x| self.generate_field(x)))
            .collect();
        let fields_named = FieldsNamed {
            brace_token: Default::default(),
            named: fields,
        };

        let item_struct = self.struct_factory.create(attrs, ident, fields_named);
        let mock_struct = MockDataStruct { item_struct };
        return mock_struct;
    }

    fn generate_with_non_camel_case_allowed(
        &self,
        mock_ident: &Ident,
        fn_infos: &[FnInfo],
    ) -> MockDataStruct {
        let mut result = self.generate(mock_ident, fn_infos);
        result
            .item_struct
            .attrs
            .push(constants::ALLOW_NON_CAMEL_CASE_TYPES_ATTRIBUTE.clone());
        return result;
    }
}

impl MockDataStructGenerator {
    const MOCK_DATA_STRUCT_IDENT_SUFFIX: &'static str = "Data";

    fn generate_field(&self, fn_info: &FnInfo) -> Field {
        let field = Field {
            attrs: Vec::new(),
            vis: Visibility::Inherited,
            mutability: FieldMutability::None,
            ident: Some(fn_info.data_field_ident.clone()),
            colon_token: Default::default(),
            ty: Type::Path(TypePath {
                qself: None,
                path: Path {
                    leading_colon: None,
                    segments: [PathSegment {
                        ident: constants::FN_DATA_TYPE_IDENT.clone(),
                        arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                            colon2_token: None,
                            lt_token: Default::default(),
                            args: [
                                GenericArgument::Type(
                                    self.type_factory
                                        .create_from_struct(&fn_info.call_struct.item_struct),
                                ),
                                GenericArgument::Type(
                                    self.type_factory.create_from_struct(
                                        &fn_info.args_checker_struct.item_struct,
                                    ),
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
            }),
        };
        return field;
    }
}
