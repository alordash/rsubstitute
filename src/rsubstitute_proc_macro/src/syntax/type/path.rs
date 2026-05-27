use crate::syntax::constants::IDENT_SEGMENTS_SEPARATOR;
use crate::syntax::path;
use syn::*;

pub(crate) fn to_ident(type_path: &TypePath) -> Ident {
    let result = path::to_ident(&type_path.path, IDENT_SEGMENTS_SEPARATOR);
    return result;
}
