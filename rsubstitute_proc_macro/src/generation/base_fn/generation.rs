use crate::common::models::*;
use crate::common::*;
use crate::generation::fn_info::models::*;
use crate::syntax::*;
use proc_macro2::Span;
use quote::format_ident;
use syn::punctuated::Punctuated;
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
    let (sig, block) = generate_core(span, fn_info, target_struct_path, base_impl, None);
    let result = ItemFn {
        attrs: fn_info
            .attributes
            .clone()
            .into_iter()
            .chain([attributes::doc_hidden(span)])
            .collect(),
        vis: Visibility::Public(Token![pub](span)),
        modifiers: FnModifiers::default(),
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
    pub maybe_mod_ident: Option<Ident>,
}
pub(crate) fn generate_associated(
    span: Span,
    AssociatedParams {
        fn_info,
        target_struct_path,
        base_impl,
        maybe_associated_items_info,
        maybe_mod_ident,
    }: AssociatedParams,
) -> ImplItemFn {
    let (mut sig, mut block) = generate_core(
        span,
        fn_info,
        target_struct_path,
        base_impl,
        maybe_mod_ident,
    );
    (sig, block) = normalization::normalize_method(sig, block);
    if let Some(associated_items_info) = maybe_associated_items_info {
        (sig, block) = normalization::normalize_associated_items(associated_items_info, sig, block);
    }
    let result = ImplItemFn {
        attrs: fn_info
            .attributes
            .clone()
            .into_iter()
            .chain([attributes::doc_hidden(span)])
            .collect(),
        vis: Visibility::Inherited,
        modifiers: FnModifiers::default(),
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
    maybe_mod_ident: Option<Ident>,
) -> (Signature, Block) {
    let source_signature = &fn_info.source_signature;
    let call_path = path::new(span, ["call"]);
    let mut generics = source_signature.generics.clone();
    let mut output = source_signature.output.clone();
    let mock_pat = if let Some(receiver) = &fn_info.maybe_self_type {
        let (is_reference, maybe_target_type_mutability, maybe_target_type) = match &receiver.kind {
            ReceiverKind::Reference(_, _, mutability) => (true, mutability.clone(), None),
            ReceiverKind::Typed(_, ty) => (false, None, Some(*ty.clone())),
            _ => (false, None, None),
        };
        let reference_mutability = maybe_target_type_mutability.or_else(|| receiver.mutability);
        let mut self_lifetime = None;
        if is_reference && let ReturnType::Type(_, return_type) = &mut output {
            let anonymous_lifetime_substitute = Lifetime {
                apostrophe: span,
                ident: Ident::new("__rs_ret", span),
            };
            r#type::replace_anonymous_lifetimes_in_references(
                return_type.as_mut(),
                &anonymous_lifetime_substitute,
            );
            self_lifetime = Some(anonymous_lifetime_substitute.clone());
            generics.params.insert(
                0,
                GenericParam::Lifetime(LifetimeParam {
                    attrs: Vec::new(),
                    lifetime: anonymous_lifetime_substitute,
                    colon_token: None,
                    bounds: Punctuated::new(),
                }),
            );
        }
        let normalized_target_type = maybe_target_type
            .map(|target_type| normalization::normalize_in_type(target_type, &target_struct_path))
            .unwrap_or_else(|| {
                Type::Path(TypePath {
                    attrs: Vec::new(),
                    qself: None,
                    path: target_struct_path,
                })
            });
        PatType {
            attrs: Vec::new(),
            pat: Box::new(Pat::Ident(PatIdent {
                attrs: Vec::new(),
                by_ref: None,
                mutability: receiver.mutability,
                ident: rsubstitute_self(span),
                subpat: None,
            })),
            colon_token: Token![:](span),
            ty: Box::new(if is_reference {
                Type::Reference(TypeReference {
                    attrs: Vec::new(),
                    and_token: Token![&](span),
                    lifetime: self_lifetime,
                    mutability: reference_mutability,
                    elem: Box::new(normalized_target_type),
                })
            } else {
                normalized_target_type
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
    let mut call_struct_path = maybe_mod_ident.map_or_else(
        || fn_info.call_struct.path.clone(),
        |mod_ident| {
            let mut result = fn_info.call_struct.path.clone();
            result.segments.insert(
                0,
                PathSegment {
                    ident: mod_ident,
                    arguments: PathArguments::None,
                },
            );
            return result;
        },
    );
    rsubstitute_lifetime::revert_in_first_generic_arg(&mut call_struct_path);

    let sig = Signature {
        constness: source_signature.constness.clone(),
        asyncness: source_signature.asyncness.clone(),
        safety: source_signature.safety.clone(),
        abi: source_signature.abi.clone(),
        fn_token: Token![fn](span),
        ident: get_base_fn_ident(&fn_info.fn_ident),
        generics,
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
                    attrs: Vec::new(),
                    qself: None,
                    path: call_struct_path.clone(),
                })),
            }),
        ]),
        variadic: None,
        output,
    };

    let deconstruct_call_stmt = Local {
        attrs: Vec::new(),
        let_token: Token![let](span),
        modifiers: LocalModifiers::default(),
        pat: Pat::Struct(PatStruct {
            attrs: Vec::new(),
            qself: None,
            path: call_struct_path,
            brace_token: token::Brace(span),
            fields: fn_info
                .arguments
                .iter()
                .map(|x| FieldPat {
                    attrs: Vec::new(),
                    member: Member::Named(x.ident.clone()),
                    colon_token: None,
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
    let cast_args_stmts = fn_info.arguments.iter().map(|x| {
        let mut source_pat_type = x.source_pat_type.clone();
        let attrs = core::mem::take(&mut source_pat_type.attrs);
        Local {
            attrs,
            let_token: Token![let](span),
            modifiers: LocalModifiers::default(),
            pat: Pat::Type(source_pat_type),
            init: Some(LocalInit {
                eq_token: Token![=](span),
                expr: Box::new(Expr::Macro(transmute_lifetime_expr::new(Expr::Path(
                    ExprPath {
                        attrs: Vec::new(),
                        qself: None,
                        path: path::from_ident(x.ident.clone()),
                    },
                )))),
                diverge: None,
            }),
            semi_token: Token![;](span),
        }
    });

    let normalized_base_impl = normalization::normalize_super_paths(*base_impl);
    let stmts = core::iter::once(Stmt::Local(deconstruct_call_stmt))
        .chain(cast_args_stmts.map(Stmt::Local))
        .chain(normalized_base_impl.stmts)
        .collect();
    let block = Block {
        brace_token: token::Brace(span),
        stmts,
    };

    let result = (sig, block);
    return result;
}
