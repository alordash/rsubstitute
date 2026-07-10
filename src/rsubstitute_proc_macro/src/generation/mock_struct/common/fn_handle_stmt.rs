use crate::common::models::*;
use crate::common::*;
use crate::generation::common::*;
use crate::generation::fn_info::models::*;
use crate::generation::mock_struct::models::*;
use crate::syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) struct Params<'a> {
    pub mock_struct_path: Path,
    pub fn_info: &'a FnInfo,
    pub base_fn_kind: BaseFnKind,
    pub fn_data_var_path: ExprPath,
    pub is_static: bool,
}
pub(crate) fn generate(
    ctx: &Context,
    span: Span,
    Params {
        mock_struct_path,
        fn_info,
        base_fn_kind,
        fn_data_var_path,
        is_static,
    }: Params,
) -> ExprMethodCall {
    let mock_arg = if is_static {
        void_tuple(span)
    } else {
        Expr::Reference(ExprReference {
            attrs: Vec::new(),
            and_token: Token![&](span),
            mutability: None,
            expr: Box::new(Expr::Struct(ExprStruct {
                attrs: Vec::new(),
                qself: None,
                path: mock_struct_path,
                brace_token: token::Brace(span),
                fields: [
                    generics_field::new_value(span),
                    data_field::new_clone_value(span),
                ]
                .into_iter()
                .collect(),
                dot2_token: None,
                rest: None,
            })),
        })
    };
    let the_call = Expr::Struct(ExprStruct {
        attrs: Vec::new(),
        qself: None,
        path: fn_info.call_struct.path.clone(),
        brace_token: token::Brace(span),
        fields: [generics_field::new_value(span)]
            .into_iter()
            .chain(fn_info.arguments.iter().map(|x| FieldValue {
                attrs: Vec::new(),
                member: Member::Named(x.ident.clone()),
                colon_token: Some(Token![:](span)),
                expr: Expr::Macro(transmute_lifetime_expr::new(Expr::Path(ExprPath {
                    attrs: Vec::new(),
                    qself: None,
                    path: path::from_ident(x.ident.clone()),
                }))),
            }))
            .collect(),
        dot2_token: None,
        rest: None,
    });
    let maybe_base_fn_path = match (ctx.support_base_calling, base_fn_kind) {
        (true, BaseFnKind::StaticFn(base_fn_ident)) => {
            Some(generate_base_fn_path(span, fn_info, base_fn_ident))
        }
        (true, BaseFnKind::Associated(base_fn_ident)) => {
            let mut base_fn_path = generate_base_fn_path(span, fn_info, base_fn_ident);
            base_fn_path.segments.insert(
                0,
                PathSegment {
                    ident: self_type_ident(span),
                    arguments: PathArguments::None,
                },
            );
            Some(base_fn_path)
        }
        (_, BaseFnKind::None) | (false, _) => None,
    };
    let maybe_base_call = maybe_base_fn_path.map(|path| {
        Expr::Path(ExprPath {
            attrs: Vec::new(),
            qself: None,
            path,
        })
    });

    let args = if let Some(base_call) = maybe_base_call {
        [mock_arg, the_call, base_call].into_iter().collect()
    } else {
        [mock_arg, the_call].into_iter().collect()
    };
    let result = ExprMethodCall {
        attrs: Vec::new(),
        receiver: Box::new(Expr::Path(fn_data_var_path)),
        dot_token: Token![.](span),
        method: Ident::new("handle", span),
        turbofish: None,
        paren_token: token::Paren(span),
        args,
    };
    return result;
}

fn generate_base_fn_path(span: Span, fn_info: &FnInfo, base_fn_ident: Ident) -> Path {
    let result = Path {
        leading_colon: None,
        segments: [PathSegment {
            ident: base_fn_ident,
            arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                colon2_token: Some(Token![::](span)),
                lt_token: Token![<](span),
                args: fn_info
                    .source_signature
                    .generics
                    .params
                    .iter()
                    .filter(|x| match x {
                        GenericParam::Lifetime(_) => false,
                        _ => true,
                    })
                    .cloned()
                    .map(generic_argument::from_param)
                    .collect(),
                gt_token: Token![>](span),
            }),
        }]
        .into_iter()
        .collect(),
    };
    return result;
}
