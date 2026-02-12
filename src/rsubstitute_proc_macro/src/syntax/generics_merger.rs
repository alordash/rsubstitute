use syn::{Generics, WhereClause};

pub trait IGenericsMerger {
    fn merge(&self, first: &Generics, second: &Generics) -> Generics;
}

pub(crate) struct GenericsMerger;

impl IGenericsMerger for GenericsMerger {
    fn merge(&self, first: &Generics, second: &Generics) -> Generics {
        let where_clause =
            self.merge_where_clause(first.where_clause.clone(), second.where_clause.clone());
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
}

impl GenericsMerger {
    fn merge_where_clause(
        &self,
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
                        .collect(),
                };
                return Some(result);
            }
        }
    }
}
