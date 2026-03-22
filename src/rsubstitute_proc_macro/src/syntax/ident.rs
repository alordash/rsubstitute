use quote::format_ident;
use syn::*;

pub(crate) fn flatten_path_to_ident(path: &Path) -> Ident {
    let parent_trait_path_idents: Vec<_> =
        path.segments.iter().map(|x| x.ident.to_string()).collect();
    let joined_parent_trait_path_idents = parent_trait_path_idents.join("_");
    return format_ident!("{joined_parent_trait_path_idents}");
}
