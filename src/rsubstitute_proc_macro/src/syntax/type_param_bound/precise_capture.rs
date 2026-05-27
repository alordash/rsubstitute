use crate::syntax::constants::{COMMA, GREATER, LESS};
use quote::format_ident;
use syn::*;

pub mod captured_param;

pub(crate) fn to_ident(precise_capture: &PreciseCapture) -> Ident {
    let captured_params_idents_strings: Vec<_> = precise_capture
        .params
        .iter()
        .map(captured_param::to_ident)
        .map(|x| x.to_string())
        .collect();
    let captured_params_idents_string = captured_params_idents_strings.join(COMMA);
    let result = format_ident!("use{LESS}{captured_params_idents_string}{GREATER}");
    return result;
}
