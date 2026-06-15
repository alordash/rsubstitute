mod mock_data_impl;

use crate::generation::mock_controls::models::*;
use crate::generation::r#fn::models::*;
use crate::syntax::*;
use proc_macro2::Span;
use quote::format_ident;
use syn::*;

pub(crate) struct Params<'a> {
    pub source_span: Span,
    pub target_ident: Ident,
    pub mock_type: Type,
    pub fn_infos: &'a [FnInfo],
    pub support_base_calling: bool,
    pub store_mock_data: bool,
}

pub(crate) fn generate(
    Params {
        source_span,
        target_ident,
        mock_type,
        fn_infos,
        support_base_calling,
        store_mock_data,
    }: Params,
) -> MockDataStruct {
    let fields_named = generate_fields(
        source_span,
        mock_type,
        fn_infos,
        support_base_calling,
        store_mock_data,
    );

    let item_struct = ItemStruct {
        attrs: Vec::new(),
        vis: Visibility::Public(Token![pub](source_span)),
        struct_token: Token![struct](source_span),
        ident: format_ident!("{target_ident}MockData"),
        generics: Generics::default(),
        fields: Fields::Named(fields_named),
        semi_token: None,
    };

    let path = path::from_ident(item_struct.ident.clone());
    let mock_data_impl = mock_data_impl::generate(
        source_span,
        fn_infos,
        Type::Path(TypePath {
            qself: None,
            path: path.clone(),
        }),
    );

    let result = MockDataStruct {
        path,
        item_struct,
        mock_data_impl,
    };

    return result;
}

fn generate_fields(
    source_span: Span,
    mock_type: Type,
    fn_infos: &[FnInfo],
    support_base_calling: bool,
    store_mock_data: bool,
) -> FieldsNamed {
    let result = FieldsNamed {
        brace_token: token::Brace(source_span),
        named: fn_infos
            .iter()
            .map(|fn_info| {
                let span = fn_info.syntax.spans.inputs;
                let result = Field {
                    attrs: Vec::new(),
                    vis: Visibility::Inherited,
                    mutability: FieldMutability::None,
                    ident: Some(fn_info.syntax.fn_ident.clone()),
                    colon_token: Some(Token![:](span)),
                    ty: Type::Path(TypePath {
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments: punctuated([PathSegment {
                                ident: Ident::new("FnData", span),
                                arguments: PathArguments::AngleBracketed(
                                    AngleBracketedGenericArguments {
                                        colon2_token: None,
                                        lt_token: Token![<](span),
                                        args: punctuated([
                                            GenericArgument::Lifetime(static_lifetime(span)),
                                            GenericArgument::Type(mock_type.clone()),
                                            fn_data_bool(span, support_base_calling),
                                            fn_data_bool(span, store_mock_data),
                                        ]),
                                        gt_token: Token![>](span),
                                    },
                                ),
                            }]),
                        },
                    }),
                };

                return result;
            })
            .collect(),
    };

    return result;
}

fn fn_data_bool(span: Span, value: bool) -> GenericArgument {
    let result = GenericArgument::Const(Expr::Lit(ExprLit {
        attrs: Vec::new(),
        lit: Lit::Bool(LitBool::new(value, span)),
    }));

    return result;
}
