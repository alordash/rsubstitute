use crate::mock_generation::mock_parts_generation::models::*;
use crate::mock_generation::parameters::*;
use crate::mock_generation::*;
use crate::syntax::generics;
use syn::*;

pub(crate) fn generate(
    source_generics: &Generics,
    target: Target,
    maybe_associated_generics: Option<&AssociatedGenerics>,
) -> MockGenerics {
    let mut modified_source_generics = source_generics.clone();
    let mut associated_params_count = 0;
    if let Some(associated_generics) = maybe_associated_generics {
        associated_params_count = associated_generics.generics_params.len();
        modified_source_generics
            .params
            .extend(associated_generics.generics_params.clone());
    }
    match target {
        Target::Static => (),
        _ => associated_params_count += source_generics.params.len(),
    };
    let phantom_fields = match target {
        Target::Trait => modified_source_generics
            .params
            .iter()
            .filter_map(phantom_field::try_map_generic_param)
            .collect(),
        Target::Static => Vec::new(),
    };
    let impl_generics_without_default_values =
        generics::remove_default_values(modified_source_generics.clone());
    let mock_generics = MockGenerics {
        source_generics: source_generics.clone(),
        impl_generics: modified_source_generics,
        impl_generics_without_default_values,
        phantom_fields,
        associated_params_count,
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
