use crate::syntax::*;
use proc_macro2::Span;
use quote::format_ident;
use syn::*;

pub(crate) fn replace(span: Span, mock_struct_path: Path, item_impl: &mut ItemImpl) {
    let static_fns = item_impl
        .items
        .iter_mut()
        .filter_map(|x| match x {
            ImplItem::Fn(impl_item_fn) => Some(impl_item_fn),
            _ => None,
        })
        .filter(|x| !signature::is_associated(&x.sig));
    for static_fn in static_fns {
        let args = static_fn
            .sig
            .inputs
            .iter_mut()
            .enumerate()
            .filter_map(|(i, fn_arg)| match fn_arg {
                FnArg::Typed(typed) => {
                    let arg_ident = format_ident!("__arg{i}");
                    let arg_path = ExprPath {
                        attrs: Vec::new(),
                        qself: None,
                        path: path::from_ident(arg_ident),
                    };
                    typed.pat = Box::new(Pat::Path(arg_path.clone()));
                    Some(arg_path)
                }
                _ => None,
            })
            .map(Expr::Path)
            .collect();
        let func = ExprPath {
            attrs: Vec::new(),
            qself: None,
            path: Path {
                leading_colon: None,
                segments: mock_struct_path
                    .segments
                    .iter()
                    .cloned()
                    .chain(core::iter::once(PathSegment {
                        ident: static_fn.sig.ident.clone(),
                        arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                            colon2_token: Some(Token![::](span)),
                            lt_token: Token![<](span),
                            args: static_fn
                                .sig
                                .generics
                                .params
                                .iter()
                                .map(|x| generic_argument::from_param(x.clone()))
                                .collect(),
                            gt_token: Token![>](span),
                        }),
                    }))
                    .collect(),
            },
        };
        let call_stmt = ExprCall {
            attrs: Vec::new(),
            func: Box::new(Expr::Path(func)),
            paren_token: token::Paren(span),
            args,
        };
        static_fn.block = Block {
            brace_token: static_fn.block.brace_token,
            stmts: vec![Stmt::Expr(Expr::Call(call_stmt), None)],
        };
    }
}
