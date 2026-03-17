use crate::constants;
use crate::mock_generation::mock_parts_generation::models::*;
use crate::mock_generation::*;
use syn::punctuated::Punctuated;
use syn::*;

pub(crate) fn generate(
    source_generics: &Generics,
    maybe_associated_generics: Option<&AssociatedGenerics>,
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
    if let Some(associated_generics) = maybe_associated_generics {
        for generics_param in associated_generics.generics_params.iter().rev() {
            modified_source_generics
                .params
                .insert(1, generics_param.clone());
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

// TODO - test mock generation
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
