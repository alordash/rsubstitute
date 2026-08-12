use crate::syntax::*;
use std::collections::{HashMap, HashSet};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::visit::Visit;
use syn::*;

pub(crate) struct SplitGenerics {
    pub trait_generics: Generics,
    pub target_generics: Generics,
    pub trait_where_predicates: Vec<WherePredicate>,
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

    let mut trait_where_type_predicates_map: HashMap<Ident, Vec<TypeParamBound>> = HashMap::new();
    let mut trait_where_lifetime_predicates_map: HashMap<Ident, Vec<Lifetime>> = HashMap::new();
    let target_generic_params_idents: HashSet<_> = trait_generics_searcher
        .found_generic_params
        .iter()
        .map(|x| generic_param::get_ident(x).clone())
        .collect();

    for target_generic_param in target_generics_searcher.found_generic_params.iter_mut() {
        if let GenericParam::Type(type_param) = target_generic_param {
            let bounds = &mut type_param.bounds;
            for i in (0..bounds.len()).rev() {
                let mut idents_searcher = IdentsSearcher::new(&target_generic_params_idents);
                idents_searcher.visit_type_param_bound(&bounds[i]);
                if idents_searcher.found {
                    let trait_where_predicate = trait_where_type_predicates_map
                        .entry(type_param.ident.clone())
                        .or_insert(Vec::new());
                    // SAFETY: `bounds` non emptiness is guaranteed by `for i in (0..bounds.len()).rev()` loop
                    let bound = unsafe { bounds.pop().unwrap_unchecked() };
                    trait_where_predicate.push(bound);
                }
            }
        } else if let GenericParam::Lifetime(lifetime_param) = target_generic_param {
            let lifetimes = &mut lifetime_param.bounds;
            for i in (0..lifetimes.len()).rev() {
                let mut idents_searcher = IdentsSearcher::new(&target_generic_params_idents);
                idents_searcher.visit_lifetime(&lifetimes[i]);
                if idents_searcher.found {
                    let trait_where_predicate = trait_where_lifetime_predicates_map
                        .entry(lifetime_param.lifetime.ident.clone())
                        .or_insert(Vec::new());
                    // SAFETY: `lifetimes` non emptiness is guaranteed by `for i in (0..lifetimes.len()).rev()` loop
                    let lifetime = unsafe { lifetimes.pop().unwrap_unchecked() };
                    trait_where_predicate.push(lifetime);
                }
            }
        }
    }
    let trait_where_predicates = trait_where_type_predicates_map
        .into_iter()
        .map(|(k, v)| {
            WherePredicate::Type(PredicateType {
                attrs: Vec::new(),
                lifetimes: None,
                bounded_ty: Type::Path(TypePath {
                    attrs: Vec::new(),
                    qself: None,
                    path: path::from_ident(k.clone()),
                }),
                colon_token: Token![:](k.span()),
                bounds: v.into_iter().collect(),
            })
        })
        .chain(
            trait_where_lifetime_predicates_map
                .into_iter()
                .map(|(k, v)| {
                    let span = k.span();
                    WherePredicate::Lifetime(PredicateLifetime {
                        attrs: Vec::new(),
                        lifetime: Lifetime {
                            apostrophe: span,
                            ident: k,
                        },
                        colon_token: Token![:](span),
                        bounds: v.into_iter().collect(),
                    })
                }),
        )
        .collect();
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
        trait_where_predicates,
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

struct IdentsSearcher<'a> {
    searched_idents: &'a HashSet<Ident>,
    found: bool,
}

impl<'a> IdentsSearcher<'a> {
    pub fn new(searched_idents: &'a HashSet<Ident>) -> Self {
        Self {
            searched_idents,
            found: false,
        }
    }
}

impl<'a> Visit<'_> for IdentsSearcher<'a> {
    fn visit_ident(&mut self, i: &Ident) {
        if !self.found && self.searched_idents.contains(i) {
            self.found = true;
        }
    }
}
