use crate::constants;
use crate::mock_generation::mock_parts_generation::models::*;
use crate::mock_generation::mock_parts_generation::*;
use syn::punctuated::Punctuated;
use syn::*;

pub(crate) fn generate(
    mock_struct: &MockStruct,
    mock_data_struct: &MockDataStruct,
    mock_setup_struct: &MockSetupStruct,
    mock_received_struct: &MockReceivedStruct,
    mock_type: &MockType,
) -> MockStructDefaultImpl {
    let self_ty = mock_struct.ty.clone();
    let block = mock_constructor_block::generate(
        mock_struct,
        mock_data_struct,
        mock_setup_struct,
        mock_received_struct,
        Vec::new(),
        None,
    );
    let default_fn = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        defaultness: None,
        sig: Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: Default::default(),
            ident: constants::DEFAULT_FN_IDENT.clone(),
            generics: Generics::default(),
            paren_token: Default::default(),
            inputs: Punctuated::new(),
            variadic: None,
            output: ReturnType::Type(Default::default(), Box::new(constants::SELF_TYPE.clone())),
        },
        block,
    };
    let item_impl = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Default::default(),
        generics: mock_type
            .generics
            .impl_generics_without_default_values
            .clone(),
        trait_: Some((
            None,
            constants::DEFAULT_TRAIT_PATH.clone(),
            Default::default(),
        )),
        self_ty: Box::new(self_ty),
        brace_token: Default::default(),
        items: vec![ImplItem::Fn(default_fn)],
    };
    let mock_struct_default_impl = MockStructDefaultImpl { item_impl };
    return mock_struct_default_impl;
}
