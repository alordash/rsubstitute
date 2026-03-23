use crate::constants;
use crate::mock_generation::mock_parts_generation::models::MockGenerics;
use crate::syntax::extensions::IGenericsExtensions;
use syn::*;

pub(crate) fn add_mutual_lifetime_bounds(
    generics: Generics,
    mock_generics: &MockGenerics,
) -> Generics {
    let other_lifetimes: Vec<_> = generics
        .lifetimes()
        .chain(mock_generics.impl_generics.lifetimes())
        .filter(|x| x.lifetime.ident != constants::PLACEHOLDER_LIFETIME_NAME)
        .collect();
    let placeholder_lifetime_predicate = WherePredicate::Lifetime(PredicateLifetime {
        lifetime: constants::PLACEHOLDER_LIFETIME.clone(),
        colon_token: Default::default(),
        bounds: other_lifetimes.iter().map(|x| x.lifetime.clone()).collect(),
    });
    let other_lifetimes_predicates = other_lifetimes.into_iter().map(|x| {
        WherePredicate::Lifetime(PredicateLifetime {
            lifetime: x.lifetime.clone(),
            colon_token: Default::default(),
            bounds: [constants::PLACEHOLDER_LIFETIME.clone()]
                .into_iter()
                .collect(),
        })
    });
    let all_lifetimes_predicates = core::iter::once(placeholder_lifetime_predicate)
        .chain(other_lifetimes_predicates)
        .collect();

    return generics.with_where_predicates(all_lifetimes_predicates);
}
