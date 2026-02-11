use syn::{Generics, WhereClause};

pub trait IGenericsMerger {
    fn merge(&self, base: &mut Generics, merged: &Generics);
}

pub(crate) struct GenericsMerger;

impl IGenericsMerger for GenericsMerger {
    // TODO - properly merge same type params and where clauses
    fn merge(&self, base: &mut Generics, merged: &Generics) {
        let where_clause =
            self.merge_where_clause(base.where_clause.clone(), merged.where_clause.clone());
        base.params
        
        let params = base
            .params
            .clone()
            .into_iter()
            .chain(merged.params.clone().into_iter())
            .collect();
        base.params = params;
        base.where_clause = where_clause;
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
