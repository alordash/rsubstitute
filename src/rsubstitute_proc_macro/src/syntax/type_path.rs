use crate::syntax::path;
use syn::*;

pub(crate) fn to_ident(type_path: &TypePath, separator: &str) -> Ident {
    let result = path::to_ident(&type_path.path, separator);
    return result;
}
