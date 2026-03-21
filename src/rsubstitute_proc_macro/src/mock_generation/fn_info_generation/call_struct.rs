use crate::mock_generation::fn_info_generation::models::*;
use crate::mock_generation::fn_info_generation::*;
use crate::mock_generation::mock_parts_generation::models::*;
use crate::mock_generation::models::*;
use crate::syntax::*;
use crate::*;
use quote::format_ident;
use syn::*;

// TODO - add #[repr(C)] to generated CallStruct and ArgsCheckerStruct
pub(crate) fn generate(ctx: &Ctx, fn_decl: &FnDecl, mock_generics: &MockGenerics) -> CallStruct {
    let attrs = vec![
        constants::DOC_HIDDEN_ATTRIBUTE.clone(),
        generate_call_derive_traits_attribute(ctx),
    ];
    let ident = format_ident!("{}_{}", fn_decl.get_full_ident(), CALL_STRUCT_SUFFIX);
    let fn_field_infos: Vec<_> = fn_decl
        .arguments
        .iter()
        .enumerate()
        .flat_map(|(i, x)| try_convert_fn_arg_to_field(i, x))
        .collect();
    let mut fn_fields = Vec::with_capacity(fn_field_infos.len());
    let mut fields_maybe_actual_source_types = Vec::with_capacity(fn_fields.len());
    for fn_field_info in fn_field_infos.into_iter() {
        fn_fields.push(fn_field_info.field);
        fields_maybe_actual_source_types.push(fn_field_info.maybe_actual_source_type);
    }
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
    let ty_path = r#type::create_from_struct_path(&item_struct);
    let generics_info_provider_impl =
        generics_info_provider_impl::generate(&item_struct, mock_generics.associated_params_count);
    let args_infos_provider_trait_impl =
        call_args_infos_provider_trait_impl::generate(&item_struct);
    let args_tuple_provider_trait_impl =
        call_args_tuple_provider_trait_impl::generate(&item_struct);
    let call_struct = CallStruct {
        item_struct,
        ty_path,
        generics_info_provider_impl,
        fields_maybe_actual_source_types,
        args_infos_provider_trait_impl,
        args_tuple_provider_trait_impl,
    };

    return call_struct;
}

const CALL_STRUCT_SUFFIX: &'static str = "Call";

fn generate_call_derive_traits_attribute(ctx: &Ctx) -> Attribute {
    let mut arguments = vec![];
    if ctx.support_base_calling {
        arguments.push(constants::CLONE_FOR_RSUBSTITUTE_TRAIT_NAME);
    }
    let arguments_str = arguments.join(", ");
    let derive_attribute = attribute::create(constants::DERIVE_IDENT.clone(), &arguments_str);
    return derive_attribute;
}

fn try_convert_fn_arg_to_field(arg_number: usize, fn_arg: &FnArg) -> Option<FieldInfo> {
    let pat_type = match fn_arg {
        FnArg::Receiver(_) => return None,
        FnArg::Typed(pat_type) => pat_type,
    };
    let mut ty = match &*pat_type.ty {
        Type::Reference(reference) if reference.mutability.is_some() => Type::Ptr(TypePtr {
            star_token: Default::default(),
            const_token: None,
            mutability: Some(Default::default()),
            elem: reference.elem.clone(),
        }),
        rest => rest.clone(),
    };
    let conversion_result = reference_to_pointer::convert_in_type(ty);
    ty = conversion_result.new_type;
    let ident = arg_ident::extract(arg_number, pat_type);

    let field = field::create(ident, ty);
    let result = FieldInfo {
        field,
        maybe_actual_source_type: conversion_result.maybe_actual_source_type,
    };
    return Some(result);
}

struct FieldInfo {
    pub field: Field,
    pub maybe_actual_source_type: Option<Type>,
}
