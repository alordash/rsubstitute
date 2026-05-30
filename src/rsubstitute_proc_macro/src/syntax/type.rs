use syn::*;

pub mod path;

pub(crate) fn to_ident(ty: &Type) -> Ident {
    let result = match ty {
        Type::Path(type_path) => path::to_ident(type_path),
        unsupported => panic!("Only path types supported."),
    };
    return result;
}
