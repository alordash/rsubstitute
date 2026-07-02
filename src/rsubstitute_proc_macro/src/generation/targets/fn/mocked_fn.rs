use crate::common::models::*;
use crate::common::*;
use crate::generation::common::*;
use crate::generation::fn_info::models::*;
use crate::syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) fn generate(
    ctx: &Context,
    source_span: Span,
    fn_info: &FnInfo,
    mock_struct_path: Path,
    maybe_base_fn_ident: Option<Ident>,
) -> ItemFn {
    let generic_arguments =
        generic_arguments::new(ctx, source_span, mock_struct_path.clone(), fn_info);
    let (fn_data_var_path, fn_data_stmt) =
        fn_data_stmt::new_static(source_span, fn_info, generic_arguments);
    let mock_arg = Expr::Reference(ExprReference {
        attrs: Vec::new(),
        and_token: Token![&](source_span),
        mutability: None,
        expr: Box::new(Expr::Struct(ExprStruct {
            attrs: Vec::new(),
            qself: None,
            path: mock_struct_path,
            brace_token: token::Brace(source_span),
            fields: [generics_field::new_value(source_span)]
                .into_iter()
                .collect(),
            dot2_token: None,
            rest: None,
        })),
    });
    let the_call = Expr::Struct(ExprStruct {
        attrs: Vec::new(),
        qself: None,
        path: fn_info.call_struct.path.clone(),
        brace_token: token::Brace(source_span),
        fields: [generics_field::new_value(source_span)]
            .into_iter()
            .chain(fn_info.syntax.arguments.iter().map(|x| FieldValue {
                attrs: Vec::new(),
                member: Member::Named(x.ident.clone()),
                colon_token: Some(Token![:](source_span)),
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
    let maybe_base_call = maybe_base_fn_ident.map(|base_fn_ident| {
        Expr::Path(ExprPath {
            attrs: Vec::new(),
            qself: None,
            path: Path {
                leading_colon: None,
                segments: [PathSegment {
                    ident: base_fn_ident,
                    arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                        colon2_token: Some(Token![::](source_span)),
                        lt_token: Token![<](source_span),
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
                        gt_token: Token![>](source_span),
                    }),
                }]
                .into_iter()
                .collect(),
            },
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
        dot_token: Token![.](source_span),
        method: Ident::new("handle", source_span),
        turbofish: None,
        paren_token: token::Paren(source_span),
        args,
    });

    let block = Block {
        brace_token: token::Brace(source_span),
        stmts: vec![Stmt::Local(fn_data_stmt), Stmt::Expr(handle_stmt, None)],
    };

    let result = ItemFn {
        attrs: fn_info.syntax.attributes.clone(),
        vis: Visibility::Public(Token![pub](source_span)),
        sig: *fn_info.syntax.source_signature.clone(),
        block: Box::new(block),
    };
    return result;
}
