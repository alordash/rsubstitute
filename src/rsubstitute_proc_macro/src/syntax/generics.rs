mod generics_merging;

use crate::constants;
use generics_merging::*;
use syn::visit::Visit;
use syn::*;

pub(crate) fn merge(mut first: Generics, second: &Generics) -> Generics {
    let where_clause = merge_where_clause(first.where_clause, second.where_clause.clone());
    // let params = merge_params(first.params, second.params.clone());
    first.params.extend(second.params.clone());
    let result = Generics {
        lt_token: Some(Default::default()),
        // params,
        params: first.params,
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
