use crate::constants;
use crate::mock_generation::fn_info_generation::models::*;
use crate::mock_generation::mock_parts_generation::models::*;
use crate::syntax::*;
use syn::*;

pub(crate) fn generate_for_trait(mock_type: &MockType, fn_info: &FnInfo) -> TypePath {
    let ty = generate(
        mock_type,
        fn_info,
        constants::SELF_TYPE.clone(),
        OutputTypeLifetime::Derived,
        mock_type.stores_mock_data,
    );
    return ty;
}

pub(crate) fn generate_for_static(
    mock_type: &MockType,
    fn_info: &FnInfo,
    mock_setup_struct: &MockSetupStruct,
) -> TypePath {
    let owner_type = mock_setup_struct.ty.clone();
    let stores_mock_data = false;
    let ty = generate(
        mock_type,
        fn_info,
        owner_type,
        OutputTypeLifetime::Default,
        stores_mock_data,
    );
    return ty;
}

fn generate(
    mock_type: &MockType,
    fn_info: &FnInfo,
    owner_type: Type,
    output_type_lifetime: OutputTypeLifetime,
    stores_mock_data: bool,
) -> TypePath {
    let mut arg_refs_tuple = fn_info.parent.arg_refs_tuple.clone();
    reference::normalize_anonymous_lifetimes(&mut arg_refs_tuple);
    let result = TypePath {
        qself: None,
        path: Path {
            leading_colon: None,
            segments: [PathSegment {
                ident: constants::FN_TUNER_TYPE_IDENT.clone(),
                arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                    colon2_token: None,
                    lt_token: Default::default(),
                    args: [
                        GenericArgument::Lifetime(output_type_lifetime.get()),
                        GenericArgument::Type(mock_type.ty.clone()),
                        GenericArgument::Type(owner_type),
                        GenericArgument::Type(arg_refs_tuple),
                        GenericArgument::Type(fn_info.parent.get_return_value_type()),
                        GenericArgument::Const(bool_lit::create(
                            fn_info.parent.maybe_base_fn_block.is_some(),
                        )),
                        GenericArgument::Const(bool_lit::create(stores_mock_data)),
                    ]
                    .into_iter()
                    .collect(),
                    gt_token: Default::default(),
                }),
            }]
            .into_iter()
            .collect(),
        },
    };
    return result;
}
