use crate::constants;
use crate::mock_generation::fn_info_generation::models::FnInfo;
use crate::mock_generation::mock_parts_generation::models::*;
use crate::syntax::extensions::*;
use crate::syntax::*;
use proc_macro2::{Ident, Span};
use quote::format_ident;
use std::collections::HashSet;
use syn::*;

pub(crate) fn generate_input_args(fn_info: &FnInfo, skipped_fields_count: usize) -> InputArgs {
    let fields = fn_info
        .args_checker_struct
        .item_struct
        .fields
        .iter()
        .skip(skipped_fields_count);
    let fn_arg_datas: Vec<_> = fields.map(transform_field_into_fn_arg).collect();
    let mut fn_args = Vec::with_capacity(fn_arg_datas.len());
    let mut placeholder_fn_arg_lifetime_params = Vec::with_capacity(fn_arg_datas.len());
    for fn_arg_data in fn_arg_datas.into_iter() {
        fn_args.push(fn_arg_data.fn_arg);
        if let Some(placeholder_fn_arg_lifetime_param) =
            fn_arg_data.maybe_placeholder_lifetime_param
        {
            placeholder_fn_arg_lifetime_params.push(placeholder_fn_arg_lifetime_param);
        }
    }
    let result = InputArgs {
        fn_args,
        placeholder_fn_arg_lifetime_params,
    };
    return result;
}

pub(crate) fn generate_input_args_with_static_lifetimes(
    fn_info: &FnInfo,
    skipped_fields_count: usize,
) -> InputArgs {
    let mut input_args = generate_input_args(fn_info, skipped_fields_count);
    for fn_arg in input_args.fn_args.iter_mut() {
        if let FnArg::Typed(pat_type) = fn_arg {
            reference::staticify_anonymous_lifetimes(&mut pat_type.ty);
        }
    }
    return input_args;
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

fn transform_field_into_fn_arg(field: &Field) -> FnArgData {
    let field_ident = field.get_required_ident();
    let mut ty = field.ty.clone();
    let Type::Path(ref mut ty_path) = ty else {
        panic!("Input arg field should be Type::Path (Arg<'__rs, T>).")
    };
    let PathArguments::AngleBracketed(ref mut ty_path_arguments) =
        ty_path.path.segments[0].arguments
    else {
        panic!("Input arg field should have generic arguments (Arg<'__rs, T>).")
    };
    let GenericArgument::Type(ref mut actual_ty) = ty_path_arguments.args[1] else {
        panic!("Input arg field should have type as second generic argument (Arg<'__rs, T>).");
    };
    let anonymous_lifetime_parent_lifetimes =
        get_all_anonymous_lifetime_parent_lifetimes(actual_ty);
    let maybe_placeholder_lifetime_param = if anonymous_lifetime_parent_lifetimes.is_empty() {
        None
    } else {
        Some(LifetimeParam {
            attrs: Vec::new(),
            lifetime: Lifetime::new(
                &format!("__r_{}", field_ident.to_string()),
                Span::call_site(),
            ),
            colon_token: Some(Default::default()),
            bounds: anonymous_lifetime_parent_lifetimes
                .into_iter()
                .map(|parent_lifetime_ref| parent_lifetime_ref.clone())
                .collect(),
        })
    };

    let fn_arg = FnArg::Typed(PatType {
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
                        arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                            colon2_token: None,
                            lt_token: Default::default(),
                            args: [GenericArgument::Type(ty)].into_iter().collect(),
                            gt_token: Default::default(),
                        }),
                    }]
                    .into_iter()
                    .collect(),
                },
            })]
            .into_iter()
            .collect(),
        })),
    });
    let fn_arg_data = FnArgData {
        fn_arg,
        maybe_placeholder_lifetime_param,
    };
    return fn_arg_data;
}

fn get_all_anonymous_lifetime_parent_lifetimes<'a>(ty: &'a mut Type) -> HashSet<&'a mut Lifetime> {
    let mut result = HashSet::new();
    let mut last_lifetimes_chain = Vec::new();
    let mut visitor = |type_reference: &'a mut TypeReference| {
        let Some(ref mut this_lifetime) = type_reference.lifetime else {
            if !last_lifetimes_chain.is_empty() {
                result.extend(last_lifetimes_chain.drain(..));
            }
            return;
        };
        match *type_reference.elem {
            Type::Reference(_) => last_lifetimes_chain.push(this_lifetime),
            _ => last_lifetimes_chain.clear(),
        };
    };
    reference::visit_all_optional_lifetimes(ty, &mut visitor);
    return result;
}

struct FnArgData {
    pub fn_arg: FnArg,
    pub maybe_placeholder_lifetime_param: Option<LifetimeParam>,
}
