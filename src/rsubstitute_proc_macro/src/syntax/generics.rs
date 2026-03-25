use crate::constants;
use syn::visit::Visit;
use syn::*;
use syn::punctuated::Punctuated;

pub(crate) fn merge(first: &Generics, second: &Generics) -> Generics {
    let where_clause = merge_where_clause(first.where_clause.clone(), second.where_clause.clone());
    let params = first
        .params
        .clone()
        .into_iter()
        .chain(second.params.clone().into_iter())
        .collect();
    let result = Generics {
        lt_token: Some(Default::default()),
        params,
        gt_token: Some(Default::default()),
        where_clause,
    };
    return result;
}

pub(crate) fn remove_default_values(mut generics: Generics) -> Generics {
    for param in generics.params.iter_mut() {
        match param {
            GenericParam::Type(type_param) => {
                type_param.eq_token = None;
                type_param.default = None;
            }
            GenericParam::Const(const_param) => {
                const_param.eq_token = None;
                const_param.default = None;
            }
            _ => (),
        }
    }
    return generics;
}

fn merge_params(first_params: Punctuated<GenericParam, Token![,]>, second_params: Punctuated<GenericParam, Token![,]>) {
    let mut merged_params = Vec::with_capacity(first_params.len() + second_params.len());
    merged_params.extend(first_params);
    for second_param in second_params {
        
    }
}



fn merge_where_clause(
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

struct SelfIdentSearcher {
    pub references_self: bool,
    self_type_ident: Ident,
}

impl SelfIdentSearcher {
    pub fn new() -> Self {
        Self {
            references_self: false,
            self_type_ident: constants::SELF_TYPE_IDENT.clone(),
        }
    }
}

impl Visit<'_> for SelfIdentSearcher {
    fn visit_type(&mut self, i: &'_ Type) {
        if self.references_self {
            return;
        }
        visit::visit_type(self, i);
    }

    fn visit_expr(&mut self, i: &'_ Expr) {
        if self.references_self {
            return;
        }
        visit::visit_expr(self, i);
    }

    fn visit_ident(&mut self, i: &'_ Ident) {
        if self.references_self {
            return;
        }
        if *i == self.self_type_ident {
            self.references_self = true;
            return;
        }
        visit::visit_ident(self, i);
    }
}
