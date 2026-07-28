use crate::syntax::*;
use std::collections::HashMap;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::visit::Visit;
use syn::*;

// TODO - this is apparently not needed because generics for trait and struct should be extracted from their path
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
    let mut searched_generics_idents_map = generics
        .params
        .iter()
        .map(|x| (generic_param::get_ident(x).clone(), x))
        .collect();
    let mut target_generics_searcher = GenericsSearcher::new(&mut searched_generics_idents_map);
    target_generics_searcher.visit_type(target_type);
    let mut trait_generics_searcher =
        GenericsSearcher::new(&mut target_generics_searcher.searched_generics_idents_map);
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
    searched_generics_idents_map: &'a mut HashMap<Ident, &'a GenericParam>,
    found_generic_params: Punctuated<GenericParam, Token![,]>,
}

impl<'a> GenericsSearcher<'a> {
    pub fn new(
        searched_generics_idents: &'a mut HashMap<Ident, &'a GenericParam>,
    ) -> GenericsSearcher<'a> {
        Self {
            searched_generics_idents_map: searched_generics_idents,
            found_generic_params: Punctuated::new(),
        }
    }
}

impl Visit<'_> for GenericsSearcher<'_> {
    fn visit_ident(&mut self, i: &'_ Ident) {
        if let Some(target_generic_param_entry) = self.searched_generics_idents_map.remove_entry(i)
        {
            self.found_generic_params
                .push((*target_generic_param_entry.1).clone());
        }
    }
}
