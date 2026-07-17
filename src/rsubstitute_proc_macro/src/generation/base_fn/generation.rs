use crate::common::*;
use crate::generation::common::models::*;
use crate::generation::common::*;
use crate::generation::fn_info::models::*;
use crate::preparation::r#fn::models::*;
use crate::syntax::*;
use proc_macro2::Span;
use quote::format_ident;
use syn::*;

pub(crate) fn get_base_fn_ident(fn_ident: &Ident) -> Ident {
    format_ident!("__rs_base_{}", fn_ident)
}

pub(crate) struct StaticFnParams<'a> {
    pub fn_info: &'a FnInfo,
    pub target_struct_path: Path,
    pub base_impl: Box<Block>,
}
pub(crate) fn generate_static_fn(
    span: Span,
    StaticFnParams {
        fn_info,
        target_struct_path,
        base_impl,
    }: StaticFnParams,
) -> ItemFn {
    let (sig, block) = generate_core(span, fn_info, target_struct_path, base_impl);
    let result = ItemFn {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        sig,
        block: Box::new(block),
    };
    return result;
}

pub(crate) struct AssociatedParams<'a> {
    pub fn_info: &'a FnInfo,
    pub target_struct_path: Path,
    pub base_impl: Box<Block>,
    pub maybe_associated_items_info: Option<&'a AssociatedItemsInfo>, // `Some` for trait impls, `None` for struct impls
}
pub(crate) fn generate_associated(
    span: Span,
    AssociatedParams {
        fn_info,
        target_struct_path,
        base_impl,
        maybe_associated_items_info,
    }: AssociatedParams,
) -> ImplItemFn {
    let (mut sig, mut block) = generate_core(span, fn_info, target_struct_path, base_impl);
    (sig, block) = normalization::normalize_method(sig, block);
    if let Some(associated_items_info) = maybe_associated_items_info {
        (sig, block) = normalization::normalize_associated_items(associated_items_info, sig, block);
    }
    let result = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        defaultness: None,
        sig,
        block,
    };
    return result;
}

fn generate_core(
    span: Span,
    fn_info: &FnInfo,
    target_struct_path: Path,
    base_impl: Box<Block>,
) -> (Signature, Block) {
    let source_signature = &fn_info.source_signature;
    let call_path = path::new(span, ["call"]);
    let mock_pat = if let Some(receiver) = &fn_info.maybe_self_type {
        let (type_is_reference, type_mutability) = match receiver.ty.as_ref() {
            Type::Reference(reference) => (true, reference.mutability),
            _ => (false, None),
        };
        let is_reference = type_is_reference || receiver.reference.is_some();
        let mutability = type_mutability.or_else(|| receiver.mutability);
        PatType {
            attrs: Vec::new(),
            pat: Box::new(Pat::Path(PatPath {
                attrs: Vec::new(),
                qself: None,
                path: path::from_ident(rsubstitute_self(span)),
            })),
            colon_token: Token![:](span),
            // TODO - It is not always reference, it should match source signature
            // maybe use mut visitor to replace first path segment with mock struct path!
            ty: Box::new(if is_reference {
                Type::Reference(TypeReference {
                    and_token: Token![&](span),
                    lifetime: None,
                    mutability,
                    elem: Box::new(Type::Path(TypePath {
                        qself: None,
                        path: target_struct_path,
                    })),
                })
            } else {
                Type::Path(TypePath {
                    qself: None,
                    path: target_struct_path,
                })
            }),
        }
    } else {
        PatType {
            attrs: Vec::new(),
            pat: Box::new(Pat::Wild(PatWild {
                attrs: Vec::new(),
                underscore_token: Token![_](span),
            })),
            colon_token: Token![:](span),
            ty: Box::new(void_type(span)),
        }
    };
    let sig = Signature {
        constness: source_signature.constness.clone(),
        asyncness: source_signature.asyncness.clone(),
        unsafety: source_signature.unsafety.clone(),
        abi: source_signature.abi.clone(),
        fn_token: Token![fn](span),
        ident: get_base_fn_ident(&fn_info.fn_ident),
        generics: source_signature.generics.clone(),
        paren_token: token::Paren(span),
        inputs: punctuated([
            FnArg::Typed(mock_pat),
            FnArg::Typed(PatType {
                attrs: Vec::new(),
                pat: Box::new(Pat::Path(ExprPath {
                    attrs: Vec::new(),
                    qself: None,
                    path: call_path.clone(),
                })),
                colon_token: Token![:](span),
                ty: Box::new(Type::Path(TypePath {
                    qself: None,
                    path: fn_info.call_struct.path.clone(),
                })),
            }),
        ]),
        variadic: None,
        output: source_signature.output.clone(),
    };

    let deconstruct_call_stmt = Local {
        attrs: Vec::new(),
        let_token: Token![let](span),
        pat: Pat::Struct(PatStruct {
            attrs: Vec::new(),
            qself: None,
            path: fn_info.call_struct.path.clone(),
            brace_token: token::Brace(span),
            fields: fn_info
                .arguments
                .iter()
                .map(|x| FieldPat {
                    attrs: Vec::new(),
                    member: Member::Named(x.ident.clone()),
                    colon_token: Some(Token![:](span)),
                    pat: Box::new(Pat::Path(PatPath {
                        attrs: Vec::new(),
                        qself: None,
                        path: path::from_ident(x.ident.clone()),
                    })),
                })
                .collect(),
            rest: Some(PatRest {
                attrs: Vec::new(),
                dot2_token: Token![..](span),
            }),
        }),
        init: Some(LocalInit {
            eq_token: Token![=](span),
            expr: Box::new(Expr::Path(ExprPath {
                attrs: Vec::new(),
                qself: None,
                path: call_path,
            })),
            diverge: None,
        }),
        semi_token: Token![;](span),
    };
    let cast_args_stmt = Local {
        attrs: Vec::new(),
        let_token: Token![let](span),
        pat: Pat::Type(PatType {
            attrs: Vec::new(),
            pat: Box::new(Pat::Tuple(PatTuple {
                attrs: Vec::new(),
                paren_token: token::Paren(span),
                elems: fn_info
                    .arguments
                    .iter()
                    .map(|x| *x.source_pat_type.pat.clone())
                    .collect(),
            })),
            colon_token: Token![:](span),
            ty: Box::new(Type::Tuple(TypeTuple {
                paren_token: token::Paren(span),
                elems: fn_info.arguments.iter_generics_style_types().collect(),
            })),
        }),
        init: Some(LocalInit {
            eq_token: Token![=](span),
            expr: Box::new(Expr::Macro(transmute_lifetime_expr::new(Expr::Tuple(
                ExprTuple {
                    attrs: Vec::new(),
                    paren_token: token::Paren(span),
                    elems: fn_info
                        .arguments
                        .iter()
                        .map(|x| {
                            Expr::Path(ExprPath {
                                attrs: Vec::new(),
                                qself: None,
                                path: path::from_ident(x.ident.clone()),
                            })
                        })
                        .collect(),
                },
            )))),
            diverge: None,
        }),
        semi_token: Token![;](span),
    };

    let block = Block {
        brace_token: token::Brace(span),
        stmts: vec![
            Stmt::Local(deconstruct_call_stmt),
            Stmt::Local(cast_args_stmt),
            Stmt::Expr(
                Expr::Block(ExprBlock {
                    attrs: Vec::new(),
                    label: None,
                    block: *base_impl,
                }),
                None,
            ),
        ],
    };

    let result = (sig, block);
    return result;
}
