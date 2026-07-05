use crate::common::models::*;
use crate::common::*;
use crate::generation::common::*;
use crate::generation::fn_info::models::FnInfo;
use crate::syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) enum BaseFnKind {
    None,
    Static(Ident),
    Associated(Ident),
}
pub(crate) fn generate(
    ctx: &Context,
    span: Span,
    mock_struct_path: Path,
    fn_info: &FnInfo,
    base_fn_kind: BaseFnKind,
) -> Block {
    let generic_arguments = generic_arguments::new(ctx, span, mock_struct_path.clone(), fn_info);
    let (fn_data_var_path, fn_data_stmt) =
        fn_data_stmt::new_static(span, fn_info, generic_arguments);
    let mock_arg = Expr::Reference(ExprReference {
        attrs: Vec::new(),
        and_token: Token![&](span),
        mutability: None,
        expr: Box::new(Expr::Struct(ExprStruct {
            attrs: Vec::new(),
            qself: None,
            path: mock_struct_path,
            brace_token: token::Brace(span),
            fields: [generics_field::new_value(span)].into_iter().collect(),
            dot2_token: None,
            rest: None,
        })),
    });
    let the_call = Expr::Struct(ExprStruct {
        attrs: Vec::new(),
        qself: None,
        path: fn_info.call_struct.path.clone(),
        brace_token: token::Brace(span),
        fields: [generics_field::new_value(span)]
            .into_iter()
            .chain(fn_info.syntax.arguments.iter().map(|x| FieldValue {
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
    let maybe_base_fn_path = match base_fn_kind {
        BaseFnKind::None => None,
        BaseFnKind::Static(base_fn_ident) => {
            Some(generate_base_fn_path(span, fn_info, base_fn_ident))
        }
        BaseFnKind::Associated(base_fn_ident) => {
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
    let handle_stmt = Expr::MethodCall(ExprMethodCall {
        attrs: Vec::new(),
        receiver: Box::new(Expr::Path(fn_data_var_path)),
        dot_token: Token![.](span),
        method: Ident::new("handle", span),
        turbofish: None,
        paren_token: token::Paren(span),
        args,
    });

    let result = Block {
        brace_token: token::Brace(span),
        stmts: vec![Stmt::Local(fn_data_stmt), Stmt::Expr(handle_stmt, None)],
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
                    .syntax
                    .merged_generics
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
