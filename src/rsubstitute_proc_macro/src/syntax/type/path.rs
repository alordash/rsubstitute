use crate::syntax::path;
use syn::*;

pub(crate) fn to_ident(type_path: &TypePath) -> Ident {
    let result = path::to_ident(&type_path.path, TYPE_PATH_SEGMENTS_SEPARATOR);
    return result;
}

pub const TYPE_PATH_SEGMENTS_SEPARATOR: &'static str = "_";
