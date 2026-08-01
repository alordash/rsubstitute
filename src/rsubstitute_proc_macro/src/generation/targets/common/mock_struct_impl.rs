use crate::common::models::*;
use crate::common::*;
use crate::generation::fn_info::models::*;
use crate::generation::mock_struct::models::*;
use crate::generation::mock_struct::*;
use crate::generation::*;
use crate::preparation::models::*;
use crate::syntax::*;
use proc_macro2::Span;
use quote::format_ident;
use syn::punctuated::Punctuated;
use syn::*;

pub(crate) struct Params<'a> {
    pub mock_struct_path: Path,
    pub associated_fns: &'a [Ordered<FnInfo>],
    pub static_fns: &'a [Ordered<FnInfo>],
    pub generics: Generics,
    pub mod_ident: &'a Ident,
}
pub(crate) fn generate(
    ctx: &Context,
    span: Span,
    Params {
        mock_struct_path,
        associated_fns,
        static_fns,
        generics,
        mod_ident,
    }: Params,
) -> ItemImpl {
    let base_fns = if ctx.support_base_calling {
        associated_fns
            .iter()
            .chain(static_fns.iter())
            .map(|ordered| {
                ordered.clone_map(|x| {
                    try_extract_base_fn(span, mock_struct_path.clone(), &x, mod_ident)
                })
            })
            .filter_map(|ordered| match ordered.value {
                Some(x) => Some(Ordered::new(ordered.order_number, x)),
                _ => None,
            })
            .collect()
    } else {
        Vec::new()
    };
    let mut fns: Vec<_> = associated_fns
        .iter()
        .map(|ordered| {
            map_fn(
                ctx,
                mock_struct_path.clone(),
                ordered,
                mod_ident.clone(),
                false,
                false,
            )
        })
        .chain(static_fns.iter().map(|ordered| {
            map_fn(
                ctx,
                mock_struct_path.clone(),
                ordered,
                mod_ident.clone(),
                true,
                false,
            )
        }))
        .collect();
    fns.sort_by(|a, b| a.order_number.cmp(&b.order_number));
    let items = fns
        .into_iter()
        .chain(base_fns)
        .map(|x| ImplItem::Fn(x.value))
        .collect();
    let result = generate_item_impl(span, generics, mock_struct_path, items, None);
    return result;
}

pub(crate) struct ParamsForTrait<'a> {
    pub mock_struct_path: Path,
    pub associated_fns: &'a [Ordered<FnInfo>],
    pub static_fns: &'a [Ordered<FnInfo>],
    pub merged_generics: Generics,
    pub trait_path: Path,
    pub mod_ident: &'a Ident,
}
pub(crate) struct ResultForTrait {
    pub trait_impl: ItemImpl,
    pub maybe_base_trait_and_fns_impl: Option<(ItemTrait, ItemImpl)>,
}
pub(crate) fn generate_for_trait(
    ctx: &Context,
    span: Span,
    ParamsForTrait {
        mock_struct_path,
        associated_fns,
        static_fns,
        merged_generics,
        trait_path,
        mod_ident,
    }: ParamsForTrait,
) -> ResultForTrait {
    let maybe_base_trait_and_fns_impl = ctx.support_base_calling.then(|| {
        let base_fns: Vec<_> = associated_fns
            .iter()
            .chain(static_fns.iter())
            .map(|ordered| {
                ordered.clone_map(|x| {
                    try_extract_base_fn(span, mock_struct_path.clone(), &x, mod_ident)
                })
            })
            .filter_map(|ordered| match ordered.value {
                Some(x) => Some(Ordered::new(ordered.order_number, x)),
                _ => None,
            })
            .collect();
        let fn_generics = TraitItemFn {
            attrs: Vec::new(),
            modifiers: FnModifiers::default(),
            sig: Signature {
                constness: None,
                asyncness: None,
                safety: Safety::Default,
                abi: None,
                fn_token: Token![fn](span),
                ident: Ident::new("__generics", span),
                generics: Generics::default(),
                paren_token: token::Paren(span),
                inputs: Punctuated::new(),
                variadic: None,
                output: ReturnType::Type(
                    Token![->](span),
                    Box::new(generics_phantom_data::new(
                        span,
                        generics_phantom_data::Params {
                            generics: &merged_generics,
                            maybe_argument_types: None,
                        },
                    )),
                ),
            },
            default: Some(Block {
                brace_token: token::Brace(span),
                stmts: vec![Stmt::Expr(
                    Expr::Path(ExprPath {
                        attrs: Vec::new(),
                        qself: None,
                        path: path::new_global(span, ["core", "marker", "PhantomData"]),
                    }),
                    None,
                )],
            }),
            semi_token: None,
        };
        let trait_base_fns: Vec<TraitItemFn> = base_fns
            .iter()
            .map(|x| TraitItemFn {
                attrs: Vec::new(),
                modifiers: FnModifiers::default(),
                sig: x.sig.clone(),
                default: None,
                semi_token: Some(Token![;](span)),
            })
            .collect();
        let base_fn_trait = ItemTrait {
            attrs: Vec::new(),
            vis: Visibility::Public(Token![pub](span)),
            modifiers: TraitModifiers::default(),
            unsafety: None,
            trait_token: Token![trait](span),
            ident: format_ident!("__rs_base_{}", path::last_ident(&trait_path)),
            generics: merged_generics.clone(),
            colon_token: None,
            supertraits: Punctuated::new(),
            brace_token: token::Brace(span),
            items: core::iter::once(fn_generics)
                .chain(trait_base_fns)
                .map(TraitItem::Fn)
                .collect(),
        };
        let base_fn_items = base_fns
            .into_iter()
            .map(|x| ImplItem::Fn(x.value))
            .collect();
        let base_fns_impl = generate_item_impl(
            span,
            merged_generics.clone(),
            mock_struct_path.clone(),
            base_fn_items,
            Some(path::from_ident_with_generics(
                base_fn_trait.ident.clone(),
                &base_fn_trait.generics,
            )),
        );
        return (base_fn_trait, base_fns_impl);
    });
    let mut fns: Vec<_> = associated_fns
        .iter()
        .map(|ordered| {
            map_fn(
                ctx,
                mock_struct_path.clone(),
                ordered,
                mod_ident.clone(),
                false,
                true,
            )
        })
        .chain(static_fns.iter().map(|ordered| {
            map_fn(
                ctx,
                mock_struct_path.clone(),
                ordered,
                mod_ident.clone(),
                true,
                true,
            )
        }))
        .collect();
    fns.sort_by(|a, b| a.order_number.cmp(&b.order_number));
    let items = fns.into_iter().map(|x| ImplItem::Fn(x.value)).collect();
    let trait_impl = generate_item_impl(
        span,
        merged_generics,
        mock_struct_path,
        items,
        Some(trait_path),
    );
    let result = ResultForTrait {
        trait_impl,
        maybe_base_trait_and_fns_impl,
    };
    return result;
}

fn generate_item_impl(
    span: Span,
    generics: Generics,
    mock_struct_path: Path,
    items: Vec<ImplItem>,
    maybe_trait_path: Option<Path>,
) -> ItemImpl {
    let result = ItemImpl {
        attrs: Vec::new(),
        modifiers: ImplModifiers::default(),
        unsafety: None,
        impl_token: Token![impl](span),
        generics,
        trait_: maybe_trait_path.map(|trait_path| (trait_path, Token![for](span))),
        self_ty: Box::new(Type::Path(TypePath {
            attrs: Vec::new(),
            qself: None,
            path: mock_struct_path,
        })),
        brace_token: token::Brace(span),
        items,
    };
    return result;
}

fn map_fn(
    ctx: &Context,
    mock_struct_path: Path,
    ordered_fn_info: &Ordered<FnInfo>,
    mod_ident: Ident,
    is_static: bool,
    for_trait: bool,
) -> Ordered<ImplItemFn> {
    ordered_fn_info.clone_map(|fn_info| {
        let span = fn_info.spans.inputs;
        ImplItemFn {
            attrs: if !for_trait && is_static {
                fn_info
                    .attributes
                    .iter()
                    .cloned()
                    .chain(core::iter::once(attributes::doc_hidden(span)))
                    .collect()
            } else {
                fn_info.attributes.clone()
            },
            vis: fn_info.visibility.clone(),
            modifiers: FnModifiers::default(),
            sig: *fn_info.source_signature.clone(),
            block: if is_static {
                static_fn_block::generate(
                    ctx,
                    span,
                    static_fn_block::Params {
                        mock_struct_path,
                        fn_info,
                        base_fn_kind: if fn_info.maybe_base_impl.is_some() {
                            BaseFnKind::Associated(base_fn::get_base_fn_ident(&fn_info.fn_ident))
                        } else {
                            BaseFnKind::None
                        },
                        mod_ident,
                        for_struct: true,
                    },
                )
            } else {
                associated_method_block::generate(
                    ctx,
                    span,
                    associated_method_block::Params {
                        mock_struct_path,
                        fn_info,
                        maybe_base_fn_ident: if fn_info.maybe_base_impl.is_some() {
                            Some(base_fn::get_base_fn_ident(&fn_info.fn_ident))
                        } else {
                            None
                        },
                        maybe_mod_ident: Some(mod_ident),
                        for_struct: true,
                    },
                )
            },
        }
    })
}

fn try_extract_base_fn(
    span: Span,
    target_struct_path: Path,
    fn_info: &FnInfo,
    mod_ident: &Ident,
) -> Option<ImplItemFn> {
    fn_info.maybe_base_impl.clone().map(|base_impl| {
        base_fn::generate_associated(
            span,
            base_fn::AssociatedParams {
                fn_info,
                target_struct_path,
                base_impl,
                maybe_associated_items_info: None,
                maybe_mod_ident: Some(mod_ident.clone()),
            },
        )
    })
}
