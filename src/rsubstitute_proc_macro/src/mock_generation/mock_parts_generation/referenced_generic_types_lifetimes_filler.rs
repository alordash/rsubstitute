use crate::mock_generation::mock_parts_generation::models::*;
use crate::syntax::extensions::*;
use crate::syntax::*;
use proc_macro2::Ident;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use syn::visit::Visit;
use syn::*;

pub(crate) fn fill(generics: Generics, mock_type: &MockType, fn_args: &[FnArg]) -> Generics {
    let lifetimes_map: HashMap<Ident, HashSet<Lifetime>> = generics
        .type_params()
        .chain(mock_type.generics.source_generics.type_params())
        .map(|x| (x.ident.clone(), HashSet::new()))
        .collect();

    let mut referenced_generic_types_lifetimes_filler = ReferencedGenericTypesLifetimesFiller {
        generics,
        lifetimes_map,
    };

    for fn_arg in fn_args.iter() {
        referenced_generic_types_lifetimes_filler.visit_fn_arg(fn_arg);
    }

    let where_predicates: Vec<_> = referenced_generic_types_lifetimes_filler
        .lifetimes_map
        .into_iter()
        .filter_map(|(k, v)| {
            if v.is_empty() {
                None
            } else {
                Some(WherePredicate::Type(PredicateType {
                    lifetimes: None,
                    bounded_ty: r#type::create(k),
                    colon_token: Default::default(),
                    bounds: v.into_iter().map(TypeParamBound::Lifetime).collect(),
                }))
            }
        })
        .collect();
    return referenced_generic_types_lifetimes_filler
        .generics
        .with_where_predicates(where_predicates);
}

struct ReferencedGenericTypesLifetimesFiller {
    generics: Generics,
    lifetimes_map: HashMap<Ident, HashSet<Lifetime>>,
}

impl<'ast> Visit<'ast> for ReferencedGenericTypesLifetimesFiller {
    fn visit_type_reference(&mut self, type_reference: &'ast TypeReference) {
        if let Some(ref lifetime) = type_reference.lifetime
            && let Type::Path(ref elem_type_path) = *type_reference.elem
            && let Some(elem_ident) = elem_type_path.path.get_ident()
            && let Entry::Occupied(mut generic_type_entry) =
                self.lifetimes_map.entry(elem_ident.clone())
        {
            generic_type_entry.get_mut().insert(lifetime.clone());
        }

        visit::visit_type_reference(self, type_reference);
    }
}
