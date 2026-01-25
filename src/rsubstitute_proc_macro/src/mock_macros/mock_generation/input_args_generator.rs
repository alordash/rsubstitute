use crate::constants;
use crate::mock_macros::fn_info_generation::models::FnInfo;
use crate::syntax::*;
use proc_macro2::Ident;
use quote::format_ident;
use std::sync::Arc;
use syn::*;

pub trait IInputArgsGenerator {
    fn generate_input_args(&self, fn_info: &FnInfo, phantom_types_count: usize) -> Vec<FnArg>;

    fn generate_input_args_with_static_lifetimes(
        &self,
        fn_info: &FnInfo,
        phantom_types_count: usize,
    ) -> Vec<FnArg>;

    fn generate_args_checker_var_ident_and_decl_stmt(&self, fn_info: &FnInfo) -> (Ident, Stmt);
}

pub(crate) struct InputArgsGenerator {
    pub path_factory: Arc<dyn IPathFactory>,
    pub field_value_factory: Arc<dyn IFieldValueFactory>,
    pub local_factory: Arc<dyn ILocalFactory>,
    pub reference_normalizer: Arc<dyn IReferenceNormalizer>,
    pub field_checker: Arc<dyn IFieldChecker>
}

impl IInputArgsGenerator for InputArgsGenerator {
    fn generate_input_args(&self, fn_info: &FnInfo, phantom_types_count: usize) -> Vec<FnArg> {
        let result = fn_info
            .args_checker_struct
            .item_struct
            .fields
            .iter()
            .skip(1 + phantom_types_count)
            .map(|field| {
                FnArg::Typed(PatType {
                    attrs: Vec::new(),
                    pat: Box::new(Pat::Ident(PatIdent {
                        attrs: Vec::new(),
                        by_ref: None,
                        mutability: None,
                        ident: field
                            .ident
                            .clone()
                            .expect("Field in args checker struct should be named"),
                        subpat: None,
                    })),
                    colon_token: Default::default(),
                    ty: Box::new(Type::ImplTrait(TypeImplTrait {
                        impl_token: Default::default(),
                        bounds: [TypeParamBound::Trait(TraitBound {
                            paren_token: None,
                            modifier: TraitBoundModifier::None,
                            lifetimes: None,
                            path: Path {
                                leading_colon: None,
                                segments: [PathSegment {
                                    ident: constants::INTO_TRAIT_IDENT.clone(),
                                    arguments: PathArguments::AngleBracketed(
                                        AngleBracketedGenericArguments {
                                            colon2_token: None,
                                            lt_token: Default::default(),
                                            args: [GenericArgument::Type(field.ty.clone())]
                                                .into_iter()
                                                .collect(),
                                            gt_token: Default::default(),
                                        },
                                    ),
                                }]
                                .into_iter()
                                .collect(),
                            },
                        })]
                        .into_iter()
                        .collect(),
                    })),
                })
            })
            .collect();
        return result;
    }

    fn generate_input_args_with_static_lifetimes(
        &self,
        fn_info: &FnInfo,
        phantom_types_count: usize,
    ) -> Vec<FnArg> {
        let mut fn_args = self.generate_input_args(fn_info, phantom_types_count);
        for fn_arg in fn_args.iter_mut() {
            if let FnArg::Typed(pat_type) = fn_arg {
                self.reference_normalizer.staticify(&mut pat_type.ty);
            }
        }
        return fn_args;
    }

    fn generate_args_checker_var_ident_and_decl_stmt(&self, fn_info: &FnInfo) -> (Ident, Stmt) {
        let args_checker_var_ident = format_ident!(
            "{}_{}",
            fn_info.parent.ident,
            Self::ARGS_CHECKER_VARIABLE_SUFFIX
        );
        let field_values: Vec<_> = fn_info
            .args_checker_struct
            .item_struct
            .fields
            .iter()
            .map(|field| {
                if self.field_checker.is_phantom_data(field) {
                    let field_ident = field.get_required_ident();
                    return self.field_value_factory.create_as_phantom_data(field_ident);
                }
                return self.field_value_factory.create_with_into_conversion(field);
            })
            .collect();
        let args_checker_decl_stmt = Stmt::Local(self.local_factory.create(
            args_checker_var_ident.clone(),
            LocalInit {
                eq_token: Default::default(),
                expr:
                    Box::new(
                        Expr::Struct(
                            ExprStruct {
                                attrs: Vec::new(),
                                qself: None,
                                path:
                                    self.path_factory.create(
                                        fn_info.args_checker_struct.item_struct.ident.clone(),
                                    ),
                                brace_token: Default::default(),
                                fields: field_values.into_iter().collect(),
                                dot2_token: None,
                                rest: None,
                            },
                        ),
                    ),
                diverge: None,
            },
        ));
        return (args_checker_var_ident, args_checker_decl_stmt);
    }
}

impl InputArgsGenerator {
    const ARGS_CHECKER_VARIABLE_SUFFIX: &'static str = "args_checker";
}
