use crate::mock_generation::fn_info_generation::models::*;
use crate::mock_generation::fn_info_generation::*;
use crate::mock_generation::mock_parts_generation::models::*;
use crate::mock_generation::models::*;
use crate::syntax::*;
use crate::*;
use quote::format_ident;
use syn::*;

pub(crate) fn generate(
    fn_decl: &FnDecl,
    call_struct: &CallStruct,
    mock_generics: &MockGenerics,
    target: Target,
) -> ArgsCheckerStruct {
    let attrs = vec![
        constants::DOC_HIDDEN_ATTRIBUTE.clone(),
        attribute::create(constants::DERIVE_IDENT.clone(), constants::DEBUG_TRAIT_NAME),
    ];
    let ident = format_ident!(
        "{}_{}",
        fn_decl.get_full_ident(),
        ARGS_CHECKER_STRUCT_SUFFIX
    );
    let fn_fields: Vec<_> = call_struct
        .item_struct
        .fields
        .iter()
        .skip_while(|x| field::is_phantom_data(x))
        .map(convert_call_struct_field_to_arg_field)
        .collect();
    let struct_fields = mock_generics
        .phantom_fields
        .iter()
        .cloned()
        .chain(fn_decl.internal_phantom_fields.iter().cloned())
        .chain(fn_fields)
        .collect();
    let fields_named = FieldsNamed {
        brace_token: Default::default(),
        named: struct_fields,
    };

    let item_struct = r#struct::create(attrs, ident, fn_decl.merged_generics.clone(), fields_named);
    let ty_path = r#type::create_from_struct_path(&item_struct);
    let skipped_generic_params_count = match target {
        Target::Static => 0,
        _ => mock_generics.impl_generics.params.len(),
    };
    let generics_info_provider_impl = generics_info_provider_impl::generate(
        &item_struct.generics,
        Type::Path(ty_path.clone()),
        skipped_generic_params_count,
    );

    let args_checker_trait_impl = args_checker_trait_impl::generate(
        &call_struct,
        ty_path.clone(),
        item_struct.generics.clone(),
        fn_decl.get_internal_phantom_types_count() + mock_generics.get_phantom_fields_count(),
    );
    let args_checker_args_formatter_trait_impl = args_checker_args_formatter_trait_impl::generate(
        &item_struct,
        &call_struct.fields_maybe_actual_source_types,
    );

    let args_checker_struct = ArgsCheckerStruct {
        generics_info_provider_impl,
        item_struct,
        ty_path,
        args_checker_trait_impl,
        args_checker_args_formatter_trait_impl,
    };

    return args_checker_struct;
}

const ARGS_CHECKER_STRUCT_SUFFIX: &'static str = "ArgsChecker";

fn convert_call_struct_field_to_arg_field(field: &Field) -> Field {
    let mut arg_field = field.clone();
    arg_field.ty = Type::Path(arg_type::create(arg_field.ty));

    return arg_field;
}
