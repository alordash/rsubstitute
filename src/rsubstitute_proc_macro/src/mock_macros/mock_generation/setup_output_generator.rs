use crate::constants;
use crate::mock_macros::fn_info_generation::models::*;
use crate::mock_macros::mock_generation::models::*;
use crate::syntax::*;
use std::sync::Arc;
use syn::*;

pub trait ISetupOutputGenerator {
    fn generate_for_trait(&self, fn_info: &FnInfo, mock_type: &MockType) -> ReturnType;

    fn generate_for_static(
        &self,
        fn_info: &FnInfo,
        mock_type: &MockType,
        mock_setup_struct: &MockSetupStruct,
    ) -> ReturnType;
}

pub(crate) struct SetupOutputGenerator {
    pub type_factory: Arc<dyn ITypeFactory>,
}

impl ISetupOutputGenerator for SetupOutputGenerator {
    fn generate_for_trait(&self, fn_info: &FnInfo, mock_type: &MockType) -> ReturnType {
        let ty = self.generate(
            fn_info,
            mock_type,
            constants::DEFAULT_ARG_FIELD_LIFETIME.clone(),
            constants::SELF_TYPE.clone(),
        );
        let result = ReturnType::Type(Default::default(), Box::new(ty));
        return result;
    }

    fn generate_for_static(
        &self,
        fn_info: &FnInfo,
        mock_type: &MockType,
        mock_setup_struct: &MockSetupStruct,
    ) -> ReturnType {
        let owner_type = self
            .type_factory
            .create_from_struct(&mock_setup_struct.item_struct);
        let ty = self.generate(
            fn_info,
            mock_type,
            constants::DEFAULT_ARG_FIELD_LIFETIME.clone(),
            owner_type,
        );
        let result = ReturnType::Type(Default::default(), Box::new(ty));
        return result;
    }
}

impl SetupOutputGenerator {
    fn generate(
        &self,
        fn_info: &FnInfo,
        mock_type: &MockType,
        lifetime: Lifetime,
        owner_type: Type,
    ) -> Type {
        let result = Type::Path(TypePath {
            qself: None,
            path: Path {
                leading_colon: None,
                segments: [PathSegment {
                    ident: constants::FN_TUNER_TYPE_IDENT.clone(),
                    arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                        colon2_token: None,
                        lt_token: Default::default(),
                        args: [
                            GenericArgument::Lifetime(lifetime),
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
                            GenericArgument::Type(owner_type),
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
        return result;
    }
}
