use crate::constants;
use crate::mock_macros::fn_info_generation::models::*;
use crate::mock_macros::mock_generation::models::*;
use crate::syntax::*;
use std::sync::Arc;
use syn::*;

pub trait IFnSetupOutputGenerator {
    fn generate_for_struct(&self, fn_info: &FnInfo) -> ReturnType;

    fn generate_for_static(
        &self,
        fn_info: &FnInfo,
        mock_setup_struct: &MockSetupStruct,
        base_caller_struct: &BaseCallerStruct,
    ) -> ReturnType;
}

pub(crate) struct FnSetupOutputGenerator {
    pub type_factory: Arc<dyn ITypeFactory>,
    pub reference_normalizer: Arc<dyn IReferenceNormalizer>,
}

impl IFnSetupOutputGenerator for FnSetupOutputGenerator {
    fn generate_for_struct(&self, fn_info: &FnInfo) -> ReturnType {
        let ty = self.generate_with_parameters(
            fn_info,
            constants::DEFAULT_ARG_FIELD_LIFETIME.clone(),
            constants::SELF_TYPE.clone(),
            constants::VOID_TYPE.clone(),
        );
        let result = ReturnType::Type(Default::default(), Box::new(ty));
        return result;
    }

    fn generate_for_static(
        &self,
        fn_info: &FnInfo,
        mock_setup_struct: &MockSetupStruct,
        base_caller_struct: &BaseCallerStruct,
    ) -> ReturnType {
        let owner_type = self
            .type_factory
            .create_from_struct(&mock_setup_struct.item_struct);
        let base_caller_type = self
            .type_factory
            .create_from_struct(&base_caller_struct.item_struct.clone());
        let mut ty = self.generate_with_parameters(
            fn_info,
            constants::STATIC_LIFETIME.clone(),
            owner_type,
            base_caller_type,
        );
        self.reference_normalizer.staticify(&mut ty);
        let result = ReturnType::Type(Default::default(), Box::new(ty));
        return result;
    }
}

impl FnSetupOutputGenerator {
    fn generate_with_parameters(
        &self,
        fn_info: &FnInfo,
        lifetime: Lifetime,
        owner_type: Type,
        base_caller_type: Type,
    ) -> Type {
        let result = Type::Path(TypePath {
            qself: None,
            path: Path {
                leading_colon: None,
                segments: [PathSegment {
                    ident: constants::SHARED_FN_CONFIG_TYPE_IDENT.clone(),
                    arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                        colon2_token: None,
                        lt_token: Default::default(),
                        args: [
                            GenericArgument::Lifetime(lifetime),
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
                            GenericArgument::Type(base_caller_type),
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
