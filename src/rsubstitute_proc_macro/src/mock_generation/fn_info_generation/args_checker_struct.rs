use crate::mock_generation::fn_info_generation::models::*;
use crate::mock_generation::mock_parts_generation::models::*;
use crate::mock_generation::models::*;
use crate::syntax::*;
use crate::*;
use quote::format_ident;
use syn::*;

pub(crate) fn generate(fn_decl: &FnDecl, mock_generics: &MockGenerics) -> ArgsCheckerStruct {
    let attrs = vec![
        constants::DOC_HIDDEN_ATTRIBUTE.clone(),
        generate_arg_checker_derive_traits_attribute(),
    ];
    let ident = format_ident!(
        "{}_{}",
        fn_decl.get_full_ident(),
        ARGS_CHECKER_STRUCT_SUFFIX
    );
    let fn_fields: Vec<_> = fn_decl
        .arguments
        .iter()
        .enumerate()
        .flat_map(|(i, x)| try_convert_fn_arg_to_field(i, x))
        .collect();
    let struct_fields = core::iter::once(constants::DEFAULT_ARG_LIFETIME_FIELD.clone())
        .chain(mock_generics.phantom_fields.iter().cloned())
        .chain(fn_decl.internal_phantom_fields.iter().cloned())
        .chain(fn_fields)
        .collect();
    let fields_named = FieldsNamed {
        brace_token: Default::default(),
        named: struct_fields,
    };

    let mut item_struct =
        r#struct::create(attrs, ident, fn_decl.merged_generics.clone(), fields_named);
    lifetime::normalize_anonymous_lifetimes_in_struct(&mut item_struct);
    let ty = r#type::create_from_struct_path(&item_struct);
    let args_checker_struct = ArgsCheckerStruct {
        item_struct,
        ty_path: ty,
    };

    return args_checker_struct;
}

const ARGS_CHECKER_STRUCT_SUFFIX: &'static str = "ArgsChecker";

fn generate_arg_checker_derive_traits_attribute() -> Attribute {
    let derive_attribute = attribute::create(
        constants::DERIVE_IDENT.clone(),
        &format!(
            "{}, {}, {}",
            constants::DEBUG_TRAIT_NAME,
            constants::I_ARGS_FORMATTER_TRAIT_NAME,
            constants::I_GENERICS_INFO_PROVIDER_TRAIT_NAME,
        ),
    );
    return derive_attribute;
}

fn try_convert_fn_arg_to_field(arg_number: usize, fn_arg: &FnArg) -> Option<Field> {
    let pat_type = match fn_arg {
        FnArg::Receiver(_) => return None,
        FnArg::Typed(pat_type) => pat_type,
    };
    let ty = arg_type::create(*pat_type.ty.clone());
    let ident = arg_ident::extract(arg_number, pat_type);

    let result = field::create(ident, ty);
    return Some(result);
}
