use crate::mock_generation::fn_info_generation::reference_to_pointer;
use crate::mock_generation::fn_info_generation::reference_to_pointer::ConversionStrategy;
use crate::syntax::*;
use proc_macro2::Ident;
use quote::format_ident;
use syn::*;

pub(crate) fn try_map_generic_param(generic_param: &GenericParam) -> Option<Field> {
    match generic_param {
        GenericParam::Lifetime(generic_lifetime) => Some(field::create(
            format_phantom_generic_param_name(&generic_lifetime.lifetime.ident),
            r#type::phantom_data_lifetime(generic_lifetime.lifetime.clone()),
        )),
        GenericParam::Type(generic_type) => Some(field::create(
            format_phantom_generic_param_name(&generic_type.ident),
            r#type::phantom_data(generic_type.ident.clone()),
        )),
        _ => None,
    }
}

pub(crate) fn try_create_for_fn_arg(arg_number: usize, fn_arg: &FnArg) -> Option<Field> {
    match fn_arg {
        FnArg::Typed(pat_type) => {
            let mut ty = *pat_type.ty.clone();
            ty = reference_to_pointer::convert_in_type(ty, ConversionStrategy::OnlyAnonymous)
                .new_type;
            Some(field::create(
                format_phantom_arg_name(&arg_ident::extract(arg_number, pat_type)),
                r#type::wrap_in_phantom_data(ty),
            ))
        }
        _ => None,
    }
}

fn format_phantom_generic_param_name(ident: &Ident) -> Ident {
    format_ident!("_phantom_GenericParam_{ident}")
}

fn format_phantom_arg_name(ident: &Ident) -> Ident {
    format_ident!("_phantom_{ident}")
}
