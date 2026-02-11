use syn::*;

pub trait IGenericsMerger {
    fn merge(&self, base_generics: &mut Generics, merged_generics: &Generics);
}

pub(crate) struct GenericsMerger;

impl IGenericsMerger for GenericsMerger {
    // TODO - properly merge same type params and where clauses
    fn merge(&self, base_generics: &mut Generics, merged_generics: &Generics) {
        let mut extra_params = Vec::with_capacity(merged_generics.params.len());
        for merge_param in merged_generics.params.iter() {
            let maybe_existing_param = self.try_find_existing_param(base_generics, &merge_param);

            if let Some(existing_param) = maybe_existing_param {
                match existing_param {
                    ExistingParamPair::Lifetime { base, merge } => base.bounds.extend(merge.bounds),
                    ExistingParamPair::Type { .. } => {}
                    ExistingParamPair::Const { .. } => {}
                }
            }
        }

        base_generics.params.extend(extra_params);

        let where_clause = self.merge_where_clause(
            base_generics.where_clause.clone(),
            merged_generics.where_clause.clone(),
        );
        let params = base_generics
            .params
            .clone()
            .into_iter()
            .chain(merged_generics.params.clone().into_iter())
            .collect();
        base_generics.params = params;
        base_generics.where_clause = where_clause;
    }
}

impl GenericsMerger {
    #[inline(always)]
    fn try_find_existing_param<'a>(
        &self,
        base_generics: &'a mut Generics,
        merge_param: &'a GenericParam,
    ) -> Option<ExistingParamPair<'a>> {
        let maybe_existing = match merge_param {
            GenericParam::Lifetime(merge) => base_generics
                .params
                .iter_mut()
                .find_map(|base_param| match base_param {
                    GenericParam::Lifetime(base_lifetime)
                        if merge.lifetime.ident == base_lifetime.lifetime.ident =>
                    {
                        Some(base_lifetime)
                    }
                    _ => None,
                })
                .map(|base| ExistingParamPair::Lifetime { base, merge }),
            GenericParam::Type(merge) => base_generics
                .params
                .iter_mut()
                .find_map(|base_param| match base_param {
                    GenericParam::Type(base_type) if merge.ident == base_type.ident => {
                        Some(base_type)
                    }
                    _ => None,
                })
                .map(|base| ExistingParamPair::Type { base, merge }),
            GenericParam::Const(merge) => base_generics
                .params
                .iter_mut()
                .find_map(|base_param| match base_param {
                    GenericParam::Const(base_const) if merge.ident == base_const.ident => {
                        Some(base_const)
                    }
                    _ => None,
                })
                .map(|base| ExistingParamPair::Const { base, merge }),
        };
        return maybe_existing;
    }

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

enum ExistingParamPair<'a> {
    Lifetime {
        base: &'a mut LifetimeParam,
        merge: &'a LifetimeParam,
    },
    Type {
        base: &'a mut TypeParam,
        merge: &'a TypeParam,
    },
    Const {
        base: &'a mut ConstParam,
        merge: &'a ConstParam,
    },
}
