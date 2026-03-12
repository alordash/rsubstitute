use crate::constants;
use crate::mock_macros::fn_info_generation::models::*;
use crate::mock_macros::mock_generation::models::*;
use crate::mock_macros::mock_generation::*;
use syn::punctuated::Punctuated;
use syn::*;

pub(crate) fn generate(fn_info: &FnInfo, mock_type: &MockType) -> StaticFn {
    let mut generics = mock_type.generics.impl_generics.clone();
    generics.params.insert(
        0,
        GenericParam::Lifetime(LifetimeParam {
            attrs: Vec::new(),
            lifetime: constants::ANONYMOUS_LIFETIME.clone(),
            colon_token: Default::default(),
            bounds: Punctuated::new(),
        }),
    );
    let sig = Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: Default::default(),
        ident: fn_info.parent.get_full_ident(),
        generics,
        paren_token: Default::default(),
        inputs: mock_fn_inputs::generate(&fn_info.parent.arguments),
        variadic: None,
        output: fn_info.parent.return_value.clone(),
    };
    let block = mock_fn_block::generate_for_static(fn_info, mock_type);
    let item_fn = ItemFn {
        attrs: Vec::new(),
        vis: Visibility::Public(Default::default()),
        sig,
        block: Box::new(block),
    };
    let static_fn = StaticFn { item_fn };
    return static_fn;
}
