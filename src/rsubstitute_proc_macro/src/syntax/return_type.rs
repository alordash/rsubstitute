use crate::syntax::constants::{GREATER, HYPHEN, LEFT_PARENTHESES, RIGHT_PARENTHESES};
use crate::syntax::r#type;
use quote::format_ident;
use syn::*;

pub(crate) fn to_ident(return_type: &ReturnType) -> Ident {
    let type_ident = match return_type {
        ReturnType::Default => format_ident!("{LEFT_PARENTHESES}{RIGHT_PARENTHESES}"),
        ReturnType::Type(_, r#type) => r#type::to_ident(&r#type),
    };
    let result = format_ident!("{HYPHEN}{GREATER}{type_ident}");
    return result;
}
