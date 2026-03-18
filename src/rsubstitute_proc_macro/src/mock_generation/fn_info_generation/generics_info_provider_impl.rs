use crate::constants;
use crate::syntax::*;
use quote::{format_ident, ToTokens};
use std::cell::LazyCell;
use syn::punctuated::Punctuated;
use syn::*;

pub(crate) fn generate(item_struct: &ItemStruct, associated_params_count: usize) -> ItemImpl {
    let generic_params: Vec<_> = item_struct
        .generics
        .params
        .iter()
        .skip(1 + associated_params_count)
        .collect();
    let impl_items = generate_impl_items(generic_params);
    let item_impl = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Default::default(),
        generics: generics::remove_default_values(item_struct.generics.clone()),
        trait_: Some((
            None,
            constants::I_GENERICS_HASH_KEY_PROVIDER_TRAIT_PATH.clone(),
            Default::default(),
        )),
        self_ty: Box::new(r#type::create_from_struct(&item_struct)),
        brace_token: Default::default(),
        items: vec![
            impl_items.get_generic_parameter_infos,
            impl_items.hash_generics_type_ids_item,
            impl_items.hash_const_values_item,
        ],
    };

    return item_impl;
}

const GENERICS_HASHER_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("GenericsHasher"));
const HASHER_ARG_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("hasher"));
const TID_FN_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("tid"));
const CONST_HASH_FN_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("const_hash"));

const HASH_GENERICS_TYPE_IDS_FN_IDENT: LazyCell<Ident> =
    LazyCell::new(|| format_ident!("hash_generics_type_ids"));
const HASH_CONST_VALUES_FN_IDENT: LazyCell<Ident> =
    LazyCell::new(|| format_ident!("hash_const_values"));

fn generate_impl_items(generic_params: Vec<&GenericParam>) -> ImplItems {
    let vecs_capacity = generic_params.len();
    let mut type_params = Vec::with_capacity(vecs_capacity);
    let mut const_params = Vec::with_capacity(vecs_capacity);
    for generic_param in generic_params.iter() {
        match generic_param {
            GenericParam::Type(type_param) => type_params.push(type_param),
            GenericParam::Const(const_param) => const_params.push(const_param),
            _ => (),
        }
    }
    let get_generic_parameter_infos = generate_get_generic_parameter_infos(&generic_params);
    let hash_generics_type_ids_item = generate_hash_generics_type_ids_item(type_params);
    let hash_const_values_item = generate_hash_const_values_item(const_params);

    let impl_items = ImplItems {
        get_generic_parameter_infos,
        hash_generics_type_ids_item,
        hash_const_values_item,
    };
    return impl_items;
}

fn generate_get_generic_parameter_infos(generic_params: &[&GenericParam]) -> ImplItem {
    let sig = Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: Default::default(),
        ident: format_ident!("get_generic_parameter_infos"),
        generics: Default::default(),
        paren_token: Default::default(),
        inputs: [constants::REF_SELF_ARG.clone()].into_iter().collect(),
        variadic: None,
        output: ReturnType::Type(
            Default::default(),
            Box::new(r#type::wrap_in(
                r#type::create(format_ident!("GenericParameterInfo")),
                constants::VEC_TYPE_IDENT.clone(),
            )),
        ),
    };

    let generic_parameter_infos: Punctuated<Expr, Token![,]> = generic_params
        .iter()
        .filter_map(|generic_param| match generic_param {
            GenericParam::Type(type_param) => Some(expr_call::create_with_args(
                path::create_expr_with_generics(
                    constants::GENERIC_TYPE_INFO_FN_IDENT.clone(),
                    Generics::default(),
                ),
                vec![
                    str_lit::create_from_ident(&type_param.ident),
                    expr_call::create_without_args(path::create_expr_from_parts_with_generics(
                        vec![
                            format_ident!("core"),
                            format_ident!("any"),
                            format_ident!("type_name"),
                        ],
                        Generics {
                            lt_token: Some(Default::default()),
                            params: [GenericParam::Type(TypeParam {
                                attrs: Vec::new(),
                                ident: type_param.ident.clone(),
                                colon_token: None,
                                bounds: Punctuated::new(),
                                eq_token: None,
                                default: None,
                            })]
                            .into_iter()
                            .collect(),
                            gt_token: Some(Default::default()),
                            where_clause: None,
                        },
                    )),
                ],
            )),
            GenericParam::Const(const_param) => Some(expr_call::create_from_ident(
                constants::GENERIC_CONST_INFO_FN_IDENT.clone(),
                vec![
                    str_lit::create_from_ident(&const_param.ident),
                    path::create_expr(const_param.ident.clone()),
                ],
            )),
            _ => None,
        })
        .collect();

    let stmt = Stmt::Expr(
        Expr::Macro(ExprMacro {
            attrs: Vec::new(),
            mac: Macro {
                path: constants::MACRO_VEC_PATH.clone(),
                bang_token: Default::default(),
                delimiter: MacroDelimiter::Bracket(Default::default()),
                tokens: generic_parameter_infos.into_token_stream(),
            },
        }),
        None,
    );

    let impl_item_fn = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        defaultness: None,
        sig,
        block: Block {
            brace_token: Default::default(),
            stmts: vec![stmt],
        },
    };
    return ImplItem::Fn(impl_item_fn);
}

fn generate_hash_generics_type_ids_item(type_params: Vec<&TypeParam>) -> ImplItem {
    let sig = generate_hash_fn_sig(HASH_GENERICS_TYPE_IDS_FN_IDENT.clone());
    let stmts = if type_params.len() > 0 {
        let tid_exprs = type_params.iter().map(|x| generate_tid_expr(x));
        let tid_array_expr = ExprArray {
            attrs: Vec::new(),
            bracket_token: Default::default(),
            elems: tid_exprs.collect(),
        };
        let hash_method_call = expr_method_call::create_with_base_receiver(
            Expr::Array(tid_array_expr),
            Vec::new(),
            constants::HASH_FN_IDENT.clone(),
            vec![HASHER_ARG_IDENT.clone()],
        );
        let stmt = Stmt::Expr(Expr::MethodCall(hash_method_call), None);
        vec![stmt]
    } else {
        Vec::new()
    };
    let impl_item_fn = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        defaultness: None,
        sig,
        block: Block {
            brace_token: Default::default(),
            stmts,
        },
    };
    return ImplItem::Fn(impl_item_fn);
}

fn generate_hash_const_values_item(const_params: Vec<&ConstParam>) -> ImplItem {
    let sig = generate_hash_fn_sig(HASH_CONST_VALUES_FN_IDENT.clone());
    let stmts = const_params
        .iter()
        .map(|x| generate_const_hash_stmt(x))
        .collect();
    let impl_item_fn = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        defaultness: None,
        sig,
        block: Block {
            brace_token: Default::default(),
            stmts,
        },
    };
    return ImplItem::Fn(impl_item_fn);
}

fn generate_hash_fn_sig(fn_ident: Ident) -> Signature {
    let inputs = [
        constants::REF_SELF_ARG.clone(),
        FnArg::Typed(PatType {
            attrs: Vec::new(),
            pat: Box::new(Pat::Ident(PatIdent {
                attrs: Vec::new(),
                by_ref: None,
                mutability: None,
                ident: HASHER_ARG_IDENT.clone(),
                subpat: None,
            })),
            colon_token: Default::default(),
            ty: Box::new(r#type::mut_reference(
                r#type::create(GENERICS_HASHER_IDENT.clone()),
                None,
            )),
        }),
    ]
    .into_iter()
    .collect();
    let signature = Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: Default::default(),
        ident: fn_ident,
        generics: Generics::default(),
        paren_token: Default::default(),
        inputs,
        variadic: None,
        output: ReturnType::Default,
    };
    return signature;
}

fn generate_tid_expr(type_param: &TypeParam) -> Expr {
    let func = Expr::Path(ExprPath {
        attrs: Vec::new(),
        qself: None,
        path: Path {
            leading_colon: None,
            segments: [PathSegment {
                ident: TID_FN_IDENT.clone(),
                arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                    colon2_token: Some(Default::default()),
                    lt_token: Default::default(),
                    args: [GenericArgument::Type(r#type::create(
                        type_param.ident.clone(),
                    ))]
                    .into_iter()
                    .collect(),
                    gt_token: Default::default(),
                }),
            }]
            .into_iter()
            .collect(),
        },
    });
    let expr = expr_call::create_without_args(func);
    return expr;
}

fn generate_const_hash_stmt(const_param: &ConstParam) -> Stmt {
    let args = vec![
        Expr::Reference(ExprReference {
            attrs: Vec::new(),
            and_token: Default::default(),
            mutability: None,
            expr: Box::new(path::create_expr(const_param.ident.clone())),
        }),
        path::create_expr(HASHER_ARG_IDENT.clone()),
    ];
    let expr = expr_call::create_with_args(path::create_expr(CONST_HASH_FN_IDENT.clone()), args);
    let stmt = Stmt::Expr(expr, Some(Default::default()));
    return stmt;
}

struct ImplItems {
    pub get_generic_parameter_infos: ImplItem,
    pub hash_generics_type_ids_item: ImplItem,
    pub hash_const_values_item: ImplItem,
}
