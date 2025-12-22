use crate::syntax::IGenericArgumentFactory;
use proc_macro2::Ident;
use std::cell::OnceCell;
use std::rc::Rc;
use syn::*;

pub trait IPathFactory {
    fn create(&self, ident: Ident) -> Path;

    fn create_with_generics(&self, ident: Ident, generics: Generics) -> Path;
}

pub(crate) struct PathFactory {
    pub generic_argument_factory: Rc<OnceCell<Rc<dyn IGenericArgumentFactory>>>,
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
        let arguments = if generics.params.empty_or_trailing() {
            PathArguments::None
        } else {
            PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                colon2_token: None,
                lt_token: Default::default(),
                args: generics
                    .params
                    .iter()
                    .map(|x| {
                        self.generic_argument_factory
                            .get()
                            .expect("generic_argument_factory should be set at this point")
                            .create(x.clone())
                    })
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
}
