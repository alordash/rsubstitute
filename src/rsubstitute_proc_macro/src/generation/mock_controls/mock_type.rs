use crate::syntax::*;
use quote::format_ident;
use syn::*;

pub(crate) fn generate(target_ident: Ident) -> Type {
    let result = TypePath {
        qself: None,
        path: path::from_ident(format_ident!("{target_ident}MockData")),
    };

    return Type::Path(result);
}
