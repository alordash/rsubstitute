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
        let ty = self.generate(
            fn_info,
            constants::DEFAULT_ARG_FIELD_LIFETIME.clone(),
            constants::SELF_TYPE.clone(),
        );
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
        let ty = self.generate(
            fn_info,
            constants::DEFAULT_ARG_FIELD_LIFETIME.clone(),
            owner_type,
        );
        return ty;
    }
}

impl SetupOutputGenerator {
    fn generate(&self, fn_info: &FnInfo, lifetime: Lifetime, owner_type: Type) -> TypePath {
        let arg_refs_tuple = self.generate_arg_refs_tuple(&fn_info.parent.arguments);
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
                            GenericArgument::Lifetime(lifetime),
                            GenericArgument::Type(owner_type),
                            GenericArgument::Type(arg_refs_tuple),
                            GenericArgument::Type(fn_info.parent.get_return_value_type()),
                            GenericArgument::Const(Expr::Lit(ExprLit {
                                attrs: Vec::new(),
                                lit: Lit::Bool(LitBool::new(
                                    fn_info.parent.base_callable,
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

    fn generate_arg_refs_tuple(&self, fn_args: &[FnArg]) -> Type {
        let result = Type::Tuple(TypeTuple {
            paren_token: Default::default(),
            elems: fn_args
                .iter()
                .filter_map(|fn_arg| match fn_arg {
                    FnArg::Receiver(_) => None,
                    FnArg::Typed(pat_type) => Some(*pat_type.ty.clone()),
                })
                .map(|ty| self.type_factory.reference(ty, None))
                .collect(),
        });
        return result;
    }
}
