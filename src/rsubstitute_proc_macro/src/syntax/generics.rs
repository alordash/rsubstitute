use syn::*;

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
                    .collect(),
            };
            return Some(result);
        }
    }
}
