use crate::constants;
use crate::mock_macros::fn_info_generation::models::*;
use crate::mock_macros::mock_generation::models::*;
use crate::syntax::*;
use proc_macro2::Span;
use std::sync::Arc;
use syn::*;

pub trait ISetupOutputGenerator {
    fn generate_for_trait(&self, fn_info: &FnInfo) -> TypePath;

    fn generate_for_static(
        &self,
        fn_info: &FnInfo,
        mock_setup_struct: &MockSetupStruct,
    ) -> TypePath;
}

pub(crate) struct SetupOutputGenerator {
    pub type_factory: Arc<dyn ITypeFactory>,
}

impl ISetupOutputGenerator for SetupOutputGenerator {
    fn generate_for_trait(&self, fn_info: &FnInfo) -> TypePath {
        let ty = self.generate(fn_info, constants::SELF_TYPE.clone());
        return ty;
    }

    fn generate_for_static(
        &self,
        fn_info: &FnInfo,
        mock_setup_struct: &MockSetupStruct,
    ) -> TypePath {
        let owner_type = self
            .type_factory
            .create_from_struct(&mock_setup_struct.item_struct);
        let ty = self.generate(fn_info, owner_type);
        return ty;
    }
}

impl SetupOutputGenerator {
    fn generate(&self, fn_info: &FnInfo, owner_type: Type) -> TypePath {
        let result = TypePath {
            qself: None,
            path: Path {
                leading_colon: None,
                segments: [PathSegment {
                    ident: constants::FN_TUNER_TYPE_IDENT.clone(),
                    arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                        colon2_token: None,
                        lt_token: Default::default(),
                        args: [
                            GenericArgument::Lifetime(constants::DERIVED_LIFETIME.clone()),
                            GenericArgument::Type(owner_type),
                            GenericArgument::Type(fn_info.parent.arg_refs_tuple.clone()),
                            GenericArgument::Type(fn_info.parent.get_return_value_type()),
                            GenericArgument::Const(Expr::Lit(ExprLit {
                                attrs: Vec::new(),
                                lit: Lit::Bool(LitBool::new(
                                    fn_info.parent.maybe_base_fn_block.is_some(),
                                    Span::call_site(),
                                )),
                            })),
                        ]
                        .into_iter()
                        .collect(),
                        gt_token: Default::default(),
                    }),
                }]
                .into_iter()
                .collect(),
            },
        };
        return result;
    }
}
