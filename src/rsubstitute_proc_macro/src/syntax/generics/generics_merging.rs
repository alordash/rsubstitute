mod generic_param_merger;

use crate::syntax::generics::*;
use generic_param_merger::*;
use std::collections::HashMap;
use syn::punctuated::Punctuated;
use syn::*;

pub(crate) fn merge_params(
    first_params: Punctuated<GenericParam, Token![,]>,
    second_params: Punctuated<GenericParam, Token![,]>,
) -> Punctuated<GenericParam, Token![,]> {
    let capacity = HEURISTIC_MAX_GENERIC_PARAM_VEC_CAPACITY.min(first_params.len());
    let mut lifetime_params_map = HashMap::with_capacity(capacity);
    let mut type_params_map = HashMap::with_capacity(capacity);
    let mut const_params_map = HashMap::with_capacity(capacity);
    for (order, first_param) in first_params.into_iter().enumerate() {
        match first_param {
            GenericParam::Lifetime(lifetime_param) => {
                lifetime_params_map.insert(
                    lifetime_param.lifetime.ident.to_string(),
                    LifetimeParamMerger {
                        order,
                        lifetime_param,
                    },
                );
            }
            GenericParam::Type(type_param) => {
                type_params_map.insert(
                    type_param.ident.to_string(),
                    TypeParamMerger { order, type_param },
                );
            }
            GenericParam::Const(const_param) => {
                const_params_map.insert(
                    const_param.ident.to_string(),
                    ConstParamMerger { order, const_param },
                );
            }
        }
    }
    let mut rest_params = Vec::with_capacity(second_params.len());
    for second_param in second_params {
        match second_param {
            GenericParam::Lifetime(lifetime_param) => {
                if let Some(occupied) =
                    lifetime_params_map.get_mut(&lifetime_param.lifetime.ident.to_string())
                {
                    occupied.merge(lifetime_param);
                } else {
                    rest_params.push(GenericParam::Lifetime(lifetime_param));
                }
            }
            GenericParam::Type(type_param) => {
                if let Some(occupied) = type_params_map.get_mut(&type_param.ident.to_string()) {
                    occupied.merge(type_param);
                } else {
                    rest_params.push(GenericParam::Type(type_param));
                }
            }
            GenericParam::Const(const_param) => {
                if let Some(occupied) = const_params_map.get_mut(&const_param.ident.to_string()) {
                    occupied.merge(const_param);
                } else {
                    rest_params.push(GenericParam::Const(const_param));
                }
            }
        }
    }

    let mut param_mergers: Vec<_> = lifetime_params_map
        .into_values()
        .map(GenericParamMerger::from)
        .chain(type_params_map.into_values().map(GenericParamMerger::from))
        .chain(const_params_map.into_values().map(GenericParamMerger::from))
        .collect();
    param_mergers.sort_by(|a, b| a.order().cmp(&b.order()));
    let result = param_mergers
        .into_iter()
        .map(GenericParamMerger::into)
        .chain(rest_params)
        .collect();
    return result;
}

pub(crate) fn merge_where_clause(
    maybe_first: Option<WhereClause>,
    maybe_second: Option<WhereClause>,
) -> Option<WhereClause> {
    match (&maybe_first, &maybe_second) {
        (None, None) => None,
        _ => {
            let result = WhereClause {
                where_token: Default::default(),
                predicates: maybe_first
                    .map_or(Default::default(), |x| x.predicates)
                    .into_iter()
                    .chain(maybe_second.map_or(Default::default(), |x| x.predicates))
                    .filter(does_not_reference_self_type)
                    .collect(),
            };
            return Some(result);
        }
    }
}

fn does_not_reference_self_type(where_predicate: &WherePredicate) -> bool {
    let mut self_ident_searcher = SelfIdentSearcher::new();
    self_ident_searcher.visit_where_predicate(where_predicate);
    return !self_ident_searcher.references_self;
}

const HEURISTIC_MAX_GENERIC_PARAM_VEC_CAPACITY: usize = 40;
