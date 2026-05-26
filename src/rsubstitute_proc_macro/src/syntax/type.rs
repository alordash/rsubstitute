use syn::*;

pub mod array;
pub mod path;

pub(crate) fn to_ident(ty: &Type) -> Ident {
    match ty {
        Type::Array(type_array) => return array::to_ident(type_array),
        Type::BareFn(_) => {}
        Type::Group(_) => {}
        Type::ImplTrait(_) => {}
        Type::Infer(_) => {}
        Type::Macro(_) => {}
        Type::Never(_) => {}
        Type::Paren(_) => {}
        Type::Path(type_path) => return path::to_ident(type_path),
        Type::Ptr(_) => {}
        Type::Reference(_) => {}
        Type::Slice(_) => {}
        Type::TraitObject(_) => {}
        Type::Tuple(_) => {}
        Type::Verbatim(_) => {}
        _ => {}
    }
    todo!("support all variants of ty and make tests (like `impl Foo for [u8; 3]` or just `[u8]`")
}
