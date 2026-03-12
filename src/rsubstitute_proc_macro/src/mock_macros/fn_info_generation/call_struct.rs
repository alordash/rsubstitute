use crate::mock_macros::fn_info_generation::models::*;
use crate::mock_macros::mock_generation::models::*;
use crate::mock_macros::models::*;
use crate::syntax::*;
use crate::*;
use quote::format_ident;
use syn::*;

pub(crate) fn generate(ctx: &Ctx, fn_decl: &FnDecl, mock_generics: &MockGenerics) -> CallStruct {
    let attrs = vec![
        constants::DOC_HIDDEN_ATTRIBUTE.clone(),
        generate_call_derive_traits_attribute(ctx),
    ];
    let ident = format_ident!("{}_{}", fn_decl.get_full_ident(), CALL_STRUCT_SUFFIX);
    let fn_fields = fn_decl
        .arguments
        .iter()
        .enumerate()
        .flat_map(|(i, x)| try_convert_fn_arg_to_field(i, x));
    let internal_phantom_fields =
        if let Some(ref phantom_return_type) = fn_decl.maybe_phantom_return_field {
            vec![
                constants::DEFAULT_ARG_LIFETIME_FIELD.clone(),
                phantom_return_type.clone(),
            ]
        } else {
            vec![constants::DEFAULT_ARG_LIFETIME_FIELD.clone()]
        };
    let struct_fields = internal_phantom_fields
        .into_iter()
        .chain(mock_generics.phantom_fields.iter().cloned())
        .chain(fn_fields)
        .collect();
    let fields_named = FieldsNamed {
        brace_token: Default::default(),
        named: struct_fields,
    };

    let mut item_struct =
        r#struct::create(attrs, ident, fn_decl.merged_generics.clone(), fields_named);
    reference::normalize_anonymous_lifetimes_in_struct(&mut item_struct);
    let ty = r#type::create_from_struct(&item_struct);
    let call_struct = CallStruct { item_struct, ty };

    return call_struct;
}

const CALL_STRUCT_SUFFIX: &'static str = "Call";

fn generate_call_derive_traits_attribute(ctx: &Ctx) -> Attribute {
    let mut arguments = vec![
        constants::I_ARGS_INFOS_PROVIDER_TRAIT_NAME,
        constants::I_ARGS_TUPLE_PROVIDER_TRAIT_NAME,
        constants::I_GENERICS_HASH_KEY_PROVIDER_TRAIT_NAME,
    ];
    if ctx.support_base_calling {
        arguments.push(constants::CLONE_FOR_RSUBSTITUTE_TRAIT_NAME);
    }
    let arguments_str = arguments.join(", ");
    let derive_attribute = attribute::create(constants::DERIVE_IDENT.clone(), &arguments_str);
    return derive_attribute;
}

fn try_convert_fn_arg_to_field(arg_number: usize, fn_arg: &FnArg) -> Option<Field> {
    let pat_type = match fn_arg {
        FnArg::Receiver(_) => return None,
        FnArg::Typed(pat_type) => pat_type,
    };
    let ty = match &*pat_type.ty {
        Type::Reference(reference) if reference.mutability.is_some() => Type::Ptr(TypePtr {
            star_token: Default::default(),
            const_token: None,
            mutability: Some(Default::default()),
            elem: reference.elem.clone(),
        }),
        rest => rest.clone(),
    };
    let ident = arg_ident::extract(arg_number, pat_type);

    let result = field::create(ident, ty);
    return Some(result);
}
