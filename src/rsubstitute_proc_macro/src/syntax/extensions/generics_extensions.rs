use syn::*;

pub(crate) trait IGenericsExtensions {
    fn with_head_lifetime_param(self, lifetime_param: LifetimeParam) -> Self;

    fn with_where_predicates(self, where_predicates: Vec<WherePredicate>) -> Self;
}

impl IGenericsExtensions for Generics {
    fn with_head_lifetime_param(mut self, lifetime_param: LifetimeParam) -> Self {
        self.lt_token.get_or_insert(Default::default());
        self.gt_token.get_or_insert(Default::default());
        self.params
            .insert(0, GenericParam::Lifetime(lifetime_param));
        return self;
    }

    fn with_where_predicates(mut self, where_predicates: Vec<WherePredicate>) -> Self {
        match &mut self.where_clause {
            None => {
                self.where_clause = Some(WhereClause {
                    where_token: Default::default(),
                    predicates: where_predicates.into_iter().collect(),
                })
            }
            Some(existing) => existing.predicates.extend(where_predicates.into_iter()),
        }
        return self;
    }
}
