use crate::constants;
use crate::mock_generation::fn_info_generation::models::FnInfo;
use crate::syntax::extensions::*;
use crate::syntax::*;
use proc_macro2::Ident;
use quote::format_ident;
use syn::*;

pub(crate) fn generate_input_args(
    fn_info: &FnInfo,
    skipped_fields_count: usize,
    anonymize_normal_lifetimes: bool,
) -> Vec<FnArg> {
    let result = fn_info
        .args_checker_struct
        .item_struct
        .fields
        .iter()
        .skip(skipped_fields_count)
        .map(|field| {
            let mut ty = field.ty.clone();
            if anonymize_normal_lifetimes {
                reference::anonymize_normal_lifetimes(&mut ty);
            }
            FnArg::Typed(PatType {
                attrs: Vec::new(),
                pat: Box::new(Pat::Ident(PatIdent {
                    attrs: Vec::new(),
                    by_ref: None,
                    mutability: None,
                    ident: field.get_required_ident(),
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
                                        args: [GenericArgument::Type(ty)].into_iter().collect(),
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

pub(crate) fn generate_input_args_with_static_lifetimes(
    fn_info: &FnInfo,
    skipped_fields_count: usize,
) -> Vec<FnArg> {
    let anonymize_normal_lifetimes = false;
    let mut fn_args =
        generate_input_args(fn_info, skipped_fields_count, anonymize_normal_lifetimes);
    for fn_arg in fn_args.iter_mut() {
        if let FnArg::Typed(pat_type) = fn_arg {
            reference::staticify_anonymous_lifetimes(&mut pat_type.ty);
        }
    }
    return fn_args;
}

pub(crate) fn generate_args_checker_var_ident_and_decl_stmt(fn_info: &FnInfo) -> (Ident, Stmt) {
    let args_checker_var_ident = format_ident!(
        "{}_{}",
        fn_info.parent.get_full_ident(),
        ARGS_CHECKER_VARIABLE_SUFFIX
    );
    let field_values: Vec<_> = fn_info
        .args_checker_struct
        .item_struct
        .fields
        .iter()
        .map(|field| {
            if field::is_phantom_data(field) {
                let field_ident = field.get_required_ident();
                return field_value::create_as_phantom_data(field_ident);
            }
            return field_value::create_with_into_conversion(field);
        })
        .collect();
    let mut args_checker_struct_type = fn_info.args_checker_struct.ty_path.clone();
    let PathArguments::AngleBracketed(ref mut args_checker_struct_type_path_arguments) =
        args_checker_struct_type.path.segments[0].arguments
    else {
        panic!("ArgsCheckerStruct type path arguments must be AngleBracketed.")
    };
    args_checker_struct_type_path_arguments.args[0] =
        constants::ANONYMOUS_LIFETIME_GENERIC_ARGUMENT.clone();
    let args_checker_decl_stmt = Stmt::Local(local::create_with_type(
        args_checker_var_ident.clone(),
        Type::Path(args_checker_struct_type),
        LocalInit {
            eq_token: Default::default(),
            expr: Box::new(Expr::Struct(ExprStruct {
                attrs: Vec::new(),
                qself: None,
                path: path::create(fn_info.args_checker_struct.item_struct.ident.clone()),
                brace_token: Default::default(),
                fields: field_values.into_iter().collect(),
                dot2_token: None,
                rest: None,
            })),
            diverge: None,
        },
    ));
    return (args_checker_var_ident, args_checker_decl_stmt);
}

const ARGS_CHECKER_VARIABLE_SUFFIX: &'static str = "args_checker";
