use crate::constants;
use crate::syntax::constants::{IDENT_SEGMENTS_SEPARATOR, LEFT_PARENTHESES, RIGHT_PARENTHESES};
use crate::syntax::{bare_fn_arg, bound_lifetimes, ident, return_type};
use quote::format_ident;
use syn::*;

pub(crate) fn to_ident(type_bare_fn: &TypeBareFn) -> Ident {
    let maybe_lifetimes = type_bare_fn
        .lifetimes
        .as_ref()
        .map(bound_lifetimes::to_ident);

    let inputs_ident = ident::join(
        type_bare_fn.inputs.iter().map(bare_fn_arg::to_ident),
        constants::IDENTS_SEPARATOR,
    );
    let output_ident = return_type::to_ident(&type_bare_fn.output);

    let result = if let Some(lifetimes) = maybe_lifetimes {
        format_ident!(
            "{lifetimes}{IDENT_SEGMENTS_SEPARATOR}fn{LEFT_PARENTHESES}{inputs_ident}{RIGHT_PARENTHESES}{output_ident}"
        )
    } else {
        format_ident!("fn{LEFT_PARENTHESES}{inputs_ident}{RIGHT_PARENTHESES}{output_ident}")
    };
    return result;
}
