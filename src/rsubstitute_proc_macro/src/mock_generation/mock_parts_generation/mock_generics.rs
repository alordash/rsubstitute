use crate::constants;
use crate::mock_generation::mock_parts_generation::models::*;
use crate::mock_generation::*;
use syn::punctuated::Punctuated;
use syn::*;

pub(crate) fn generate(source_generics: &Generics) -> MockGenerics {
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
    let phantom_fields = source_generics
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
