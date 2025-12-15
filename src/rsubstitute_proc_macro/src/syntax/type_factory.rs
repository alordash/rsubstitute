use proc_macro2::Ident;
use syn::{Path, PathArguments, PathSegment, Type, TypePath};

pub trait ITypeFactory {
    fn create(&self, ident: Ident) -> Type;
}

pub struct TypeFactory;

impl ITypeFactory for TypeFactory {
    fn create(&self, ident: Ident) -> Type {
        let result = Type::Path(TypePath {
            qself: None,
            path: Path {
                leading_colon: None,
                segments: [PathSegment {
                    ident,
                    arguments: PathArguments::None,
                }]
                .into_iter()
                .collect(),
            },
        });
        return result;
    }
}
