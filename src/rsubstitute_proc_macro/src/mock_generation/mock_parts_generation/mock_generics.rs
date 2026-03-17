use crate::constants;
use crate::mock_generation::mock_parts_generation::models::*;
use crate::mock_generation::*;
use syn::punctuated::Punctuated;
use syn::*;

pub(crate) fn generate(
    source_generics: &Generics,
    maybe_trait_item_types: Option<&[&TraitItemType]>,
) -> MockGenerics {
    let mut modified_source_generics = source_generics.clone();
    modified_source_generics.params.insert(
        0,
        GenericParam::Lifetime(LifetimeParam {
            attrs: Vec::new(),
            lifetime: constants::DEFAULT_ARG_LIFETIME.clone(),
            colon_token: None,
            bounds: Punctuated::new(),
        }),
    );
    if let Some(trait_item_types) = maybe_trait_item_types {
        let assoc_types_generics: Vec<_> = trait_item_types
            .iter()
            .map(|x| extract_assoc_type_generics(x))
            .collect();
        for assoc_type_generics in assoc_types_generics.into_iter().rev() {
            modified_source_generics
                .params
                .insert(1, GenericParam::Type(assoc_type_generics.type_param));
            // modified_source_generics = modified_source_generics
            //     .with_where_predicates(assoc_type_generics.where_predicates);
        }
    }
    let phantom_fields = modified_source_generics
        .params
        .iter()
        .filter_map(phantom_field::try_map_generic_param)
        .collect();
    let mock_generics = MockGenerics {
        source_generics: source_generics.clone(),
        impl_generics: modified_source_generics,
        phantom_fields,
    };
    return mock_generics;
}

fn extract_assoc_type_generics(trait_item_type: &TraitItemType) -> AssocTypeGenerics {
    let (eq_token, default) = match trait_item_type.default {
        None => (None, None),
        Some(ref assoc_type_default) => (
            Some(assoc_type_default.0.clone()),
            Some(assoc_type_default.1.clone()),
        ),
    };
    let type_param = TypeParam {
        attrs: Vec::new(),
        ident: trait_item_type.ident.clone(),
        colon_token: trait_item_type.colon_token.clone(),
        bounds: trait_item_type.bounds.clone(),
        eq_token,
        default,
    };
    // let where_predicates = trait_item_type
    //     .generics
    //     .where_clause
    //     .as_ref()
    //     .map(|where_clause| where_clause.predicates.iter().cloned().collect())
    //     .unwrap_or_else(Vec::new);
    let assoc_type_generics = AssocTypeGenerics { type_param };
    return assoc_type_generics;
}

struct AssocTypeGenerics {
    pub type_param: TypeParam,
    // TODO - remove?
    // pub where_predicates: Vec<WherePredicate>,
}

// TODO - test
// trait Trait {
//     type MyType<TT>: Clone
//     where
//         Self: Sized,
//         TT: Clone,
//     = i32;
// }
//
// struct Struct<T = i32>(core::marker::PhantomData<T>);
//
// impl<T: Clone> Trait for Struct<T> {
//     type MyType<TT>
//         = T
//     where
//         TT: Clone;
// }
