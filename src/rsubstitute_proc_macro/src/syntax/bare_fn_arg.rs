use crate::syntax::r#type;
use syn::*;

pub(crate) fn to_ident(bare_fn_arg: &BareFnArg) -> Ident {
    let result = r#type::to_ident(&bare_fn_arg.ty);
    return result;
}
