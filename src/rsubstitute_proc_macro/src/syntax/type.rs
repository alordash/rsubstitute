use crate::syntax::type_path;
use syn::*;

pub(crate) fn to_ident(ty: &Type, separator: &str) -> Ident {
    match ty {
        Type::Array(_) => {}
        Type::BareFn(_) => {}
        Type::Group(_) => {}
        Type::ImplTrait(_) => {}
        Type::Infer(_) => {}
        Type::Macro(_) => {}
        Type::Never(_) => {}
        Type::Paren(_) => {}
        Type::Path(type_path) => return type_path::to_ident(type_path, separator),
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
