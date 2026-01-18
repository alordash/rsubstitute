use crate::syntax::IGenericArgumentFactory;
use proc_macro2::Ident;
use std::cell::OnceCell;
use std::sync::Arc;
use quote::ToTokens;
use syn::*;

pub trait IPathFactory {
    fn create(&self, ident: Ident) -> Path;

    fn create_with_generics(&self, ident: Ident, generics: Generics) -> Path;

    fn create_from_parts(&self, idents: &[Ident]) -> Path;
}

pub(crate) struct PathFactory {
    pub generic_argument_factory: Arc<OnceCell<Arc<dyn IGenericArgumentFactory>>>,
}

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

    fn create_with_generics(&self, ident: Ident, generics: Generics) -> Path {
        let arguments = if generics.params.is_empty() {
            PathArguments::None
        } else {
            let generic_argument_factory = self
                .generic_argument_factory
                .get()
                .expect("generic_argument_factory should be set at this point");
            PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                colon2_token: None,
                lt_token: Default::default(),
                args: generics
                    .params
                    .iter()
                    .map(|x| generic_argument_factory.create(x.clone()))
                    .collect(),
                gt_token: Default::default(),
            })
        };
        let result = Path {
            leading_colon: None,
            segments: [PathSegment { ident, arguments }].into_iter().collect(),
        };
        return result;
    }

    fn create_from_parts(&self, idents: &[Ident]) -> Path {
        let result = Path {
            leading_colon: None,
            segments: idents
                .iter()
                .map(|ident| PathSegment {
                    ident: ident.clone(),
                    arguments: PathArguments::None,
                })
                .collect(),
        };
        return result;
    }
}
