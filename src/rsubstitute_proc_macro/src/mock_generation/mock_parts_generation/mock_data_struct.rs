use crate::constants;
use crate::mock_generation::fn_info_generation::models::FnInfo;
use crate::mock_generation::mock_parts_generation::models::*;
use crate::syntax::extensions::*;
use crate::syntax::*;
use quote::format_ident;
use syn::*;

pub(crate) fn generate_for_trait(mock_type: &MockType, fn_infos: &[&FnInfo]) -> MockDataStruct {
    let attrs = vec![
        constants::DOC_HIDDEN_ATTRIBUTE.clone(),
        constants::DERIVE_MOCK_DATA_ATTRIBUTE.clone(),
    ];
    let ident = format_ident!(
        "{}{}",
        mock_type.ident.clone(),
        MOCK_DATA_STRUCT_IDENT_SUFFIX
    );
    let fn_fields: Vec<_> = fn_infos
        .iter()
        .map(|x| generate_field(x, mock_type))
        .collect();
    let field_and_fn_idents = fn_fields
        .iter()
        .zip(fn_infos)
        .map(|(x, y)| {
            (
                x.get_required_ident(),
                y.parent.get_str_literal_full_ident().clone(),
            )
        })
        .collect();
    let fields = [constants::DEFAULT_ARG_LIFETIME_FIELD.clone()]
        .into_iter()
        .chain(mock_type.generics.phantom_fields.clone())
        .chain(fn_fields)
        .into_iter()
        .collect();
    let fields_named = FieldsNamed {
        brace_token: Default::default(),
        named: fields,
    };

    let item_struct = r#struct::create(
        attrs,
        ident,
        mock_type.generics.impl_generics.clone(),
        fields_named,
    );
    let ty = r#type::create_from_struct(&item_struct);
    let mock_struct = MockDataStruct {
        item_struct,
        ty,
        field_and_fn_idents,
    };
    return mock_struct;
}

pub(crate) fn generate_for_static(mock_type: &MockType, fn_infos: &[&FnInfo]) -> MockDataStruct {
    let attrs = vec![
        constants::DOC_HIDDEN_ATTRIBUTE.clone(),
        constants::DERIVE_MOCK_DATA_ATTRIBUTE.clone(),
    ];
    let ident = format_ident!(
        "{}{}",
        mock_type.ident.clone(),
        MOCK_DATA_STRUCT_IDENT_SUFFIX
    );
    let fn_fields: Vec<_> = fn_infos
        .iter()
        .map(|x| generate_field(x, mock_type))
        .collect();
    let field_and_fn_idents = fn_fields
        .iter()
        .zip(fn_infos)
        .map(|(x, y)| {
            (
                x.get_required_ident(),
                y.parent.get_str_literal_full_ident(),
            )
        })
        .collect();
    let fields = [constants::DEFAULT_ARG_LIFETIME_FIELD.clone()]
        .into_iter()
        .chain(mock_type.generics.phantom_fields.clone())
        .chain(fn_fields)
        .collect();
    let fields_named = FieldsNamed {
        brace_token: Default::default(),
        named: fields,
    };

    let item_struct = r#struct::create(
        attrs,
        ident,
        mock_type.generics.impl_generics.clone(),
        fields_named,
    );
    let ty = r#type::create_from_struct(&item_struct);
    let mock_struct = MockDataStruct {
        item_struct,
        ty,
        field_and_fn_idents,
    };
    return mock_struct;
}

const MOCK_DATA_STRUCT_IDENT_SUFFIX: &'static str = "Data";

fn generate_field(fn_info: &FnInfo, mock_type: &MockType) -> Field {
    let ty = Type::Path(TypePath {
        qself: None,
        path: Path {
            leading_colon: None,
            segments: [PathSegment {
                ident: constants::FN_DATA_TYPE_IDENT.clone(),
                arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                    colon2_token: None,
                    lt_token: Default::default(),
                    args: [
                        GenericArgument::Lifetime(constants::DEFAULT_ARG_LIFETIME.clone()),
                        GenericArgument::Type(mock_type.ty.clone()),
                        GenericArgument::Const(bool_lit::create(
                            fn_info.parent.maybe_base_fn_block.is_some(),
                        )),
                        GenericArgument::Const(bool_lit::create(mock_type.stores_mock_data)),
                    ]
                    .into_iter()
                    .collect(),
                    gt_token: Default::default(),
                }),
            }]
            .into_iter()
            .collect(),
        },
    });
    let field = field::create_pub(fn_info.data_field_ident.clone(), ty);
    return field;
}
