use crate::generation::r#fn::models::*;
use crate::generation::*;
use crate::preparation::models::*;
use crate::syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) struct Params<'a> {
    pub ctx: &'a Context,
    pub mock_type: Type,
    pub stores_mock_data: bool,
    pub fn_info: &'a FnInfo,
}

pub(crate) fn new(
    Params {
        ctx,
        mock_type,
        stores_mock_data,
        fn_info,
    }: Params,
) -> TypePath {
    let span = fn_info.syntax.spans.inputs;

    let arg_refs_tuple = TypeTuple {
        paren_token: token::Paren(span),
        elems: fn_info
            .syntax
            .arguments
            .iter()
            .map(|argument| {
                Type::Reference(TypeReference {
                    and_token: Token![&](span),
                    lifetime: Some(anonymous_lifetime::new(span)),
                    mutability: None,
                    elem: Box::new(*argument.ref_style_type.clone()),
                })
            })
            .collect(),
    };

    let return_type = match &fn_info.syntax.return_type {
        ReturnType::Default => void_type(span),
        ReturnType::Type(_, ty) => r#type::anonymize_all_references(*ty.clone()),
    };

    let mock_arg = if stores_mock_data {
        Type::Reference(TypeReference {
            and_token: Token![&](span),
            lifetime: None,
            mutability: None,
            elem: Box::new(mock_type.clone()),
        })
    } else {
        mock_type.clone()
    };

    let result = TypePath {
        qself: None,
        path: Path {
            leading_colon: None,
            segments: punctuated([PathSegment {
                ident: Ident::new("FnConfigurator", span),
                arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                    colon2_token: None,
                    lt_token: Token![<](span),
                    args: punctuated([
                        GenericArgument::Lifetime(placeholder_lifetime::new(span)),
                        GenericArgument::Type(mock_type),
                        GenericArgument::Type(Type::Path(self_type(span))),
                        GenericArgument::Type(Type::Tuple(arg_refs_tuple)),
                        GenericArgument::Type(return_type),
                        GenericArgument::Type(mock_arg),
                        fn_data_bool(span, ctx.support_base_calling),
                        fn_data_bool(span, stores_mock_data),
                    ]),
                    gt_token: Token![>](span),
                }),
            }]),
        },
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
