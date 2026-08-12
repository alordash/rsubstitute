use crate::syntax::*;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::*;

pub(crate) fn combine(mut generics: Generics, extension: &Generics) -> Generics {
    generics.params.extend(extension.params.clone());
    if let Some(owner_generics_where_clause) = &extension.where_clause {
        generics
            .make_where_clause()
            .predicates
            .extend(owner_generics_where_clause.predicates.clone());
    }
    return generics;
}

pub(crate) fn with_prefix_lifetime(mut generics: Generics, prefix_lifetime: Lifetime) -> Generics {
    generics.params.insert(
        0,
        GenericParam::Lifetime(LifetimeParam {
            attrs: Vec::new(),
            lifetime: prefix_lifetime,
            colon_token: None,
            bounds: Punctuated::new(),
        }),
    );
    return generics;
}

pub(crate) fn with_lifetimes_tied_to(
    mut target_generics: Generics,
    source_generics: &Generics,
    tying_lifetime: Lifetime,
) -> Generics {
    let span = target_generics.span();

    let mut where_predicates_iter = source_generics
        .lifetimes()
        .filter_map(|x| {
            if x.lifetime.ident == tying_lifetime.ident {
                None
            } else {
                Some(WherePredicate::Lifetime(PredicateLifetime {
                    attrs: Vec::new(),
                    lifetime: x.lifetime.clone(),
                    colon_token: Token![:](span),
                    bounds: punctuated([tying_lifetime.clone()]),
                }))
            }
        })
        .peekable();
    if where_predicates_iter.peek().is_none() {
        return target_generics;
    }
    let where_predicates_with_tying_lifetime_iter = where_predicates_iter.chain(core::iter::once(
        WherePredicate::Lifetime(PredicateLifetime {
            attrs: Vec::new(),
            lifetime: tying_lifetime.clone(),
            colon_token: Token![:](span),
            bounds: source_generics
                .lifetimes()
                .map(|x| x.lifetime.clone())
                .collect(),
        }),
    ));
    let where_predicates: Vec<_> = where_predicates_with_tying_lifetime_iter.collect();
    target_generics
        .make_where_clause()
        .predicates
        .extend(where_predicates);

    return target_generics;
}

pub(crate) fn remove_defaults(mut generics: Generics) -> Generics {
    for param in generics.params.iter_mut() {
        match param {
            GenericParam::Type(type_param) => {
                type_param.default = None;
            }
            GenericParam::Const(const_param) => {
                const_param.default = None;
            }
            _ => {}
        }
    }
    return generics;
}
