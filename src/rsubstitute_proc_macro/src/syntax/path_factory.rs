use proc_macro2::Ident;
use syn::{Path, PathArguments, PathSegment};

pub trait IPathFactory {
    fn create(&self, ident: Ident) -> Path;
}

pub struct PathFactory;

impl IPathFactory for PathFactory {
    fn create(&self, ident: Ident) -> Path {
        let result = Path {
            leading_colon: None,
            segments: [PathSegment {
                ident,
                arguments: PathArguments::None,
            }]
            .into_iter()
            .collect(),
        };
        return result;
    }
}
