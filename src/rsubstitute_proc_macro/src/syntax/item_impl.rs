use crate::syntax::*;
use std::collections::HashSet;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::visit::Visit;
use syn::*;

pub(crate) struct SplitGenerics {
    pub target_generics: Generics,
    pub trait_generics: Generics,
}
pub(crate) fn split_generics(
    generics: &Generics,
    trait_path: &Path,
    target_type: &Type,
) -> SplitGenerics {
    let span = generics.span();
    let searched_generics_ident = generics
        .params
        .iter()
        .map(generic_param::get_ident)
        .cloned()
        .collect();
    let mut target_generics_searcher = GenericsSearcher::new(&searched_generics_ident);
    target_generics_searcher.visit_type(target_type);
    let mut trait_generics_searcher = GenericsSearcher::new(&searched_generics_ident);
    trait_generics_searcher.visit_path(trait_path);
    let result = SplitGenerics {
        target_generics: if target_generics_searcher.found_generic_params.is_empty() {
            Generics::default()
        } else {
            Generics {
                lt_token: Some(Token![<](span)),
                params: target_generics_searcher.found_generic_params,
                gt_token: Some(Token![>](span)),
                where_clause: None,
            }
        },
        trait_generics: if trait_generics_searcher.found_generic_params.is_empty() {
            Generics::default()
        } else {
            Generics {
                lt_token: Some(Token![<](span)),
                params: trait_generics_searcher.found_generic_params,
                gt_token: Some(Token![>](span)),
                where_clause: None,
            }
        },
    };
    return result;
}

struct GenericsSearcher<'a> {
    searched_generics_idents: &'a HashSet<Ident>,
    found_generic_params: Punctuated<GenericParam, Token![,]>,
}

impl<'a> GenericsSearcher<'a> {
    pub fn new(searched_generics_idents: &'a HashSet<Ident>) -> GenericsSearcher {
        Self {
            searched_generics_idents,
            found_generic_params: Punctuated::new(),
        }
    }
}

impl Visit<'_> for GenericsSearcher<'_> {
    fn visit_generic_param(&mut self, i: &'_ GenericParam) {
        let ident = generic_param::get_ident(i);
        if self.searched_generics_idents.contains(ident) {
            self.found_generic_params.push(i.clone())
        }
    }
}
