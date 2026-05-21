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
