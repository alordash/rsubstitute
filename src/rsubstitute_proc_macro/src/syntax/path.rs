use crate::syntax::ident;
use syn::*;

pub(crate) fn to_ident(path: &Path, separator: &str) -> Ident {
    let segments_idents = path.segments.iter().map(|x| &x.ident);
    let ident = ident::join(segments_idents, separator);
    return ident;
}
