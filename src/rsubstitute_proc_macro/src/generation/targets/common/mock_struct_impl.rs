use crate::common::models::*;
use crate::generation::fn_info::models::*;
use crate::generation::mock_struct::models::*;
use crate::generation::mock_struct::*;
use crate::generation::*;
use crate::preparation::models::*;
use proc_macro2::Span;
use syn::*;

pub(crate) struct Params<'a> {
    pub mock_struct_path: Path,
    pub associated_fns: &'a [Ordered<FnInfo>],
    pub static_fns: &'a [Ordered<FnInfo>],
    pub generics: Generics,
}
pub(crate) fn generate(
    ctx: &Context,
    span: Span,
    Params {
        mock_struct_path,
        associated_fns,
        static_fns,
        generics,
    }: Params,
) -> ItemImpl {
    let base_fns = if ctx.support_base_calling {
        associated_fns
            .iter()
            .chain(static_fns.iter())
            .map(|ordered| {
                ordered.clone_map(|x| try_extract_base_fn(span, mock_struct_path.clone(), &x))
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
        .map(|ordered| map_fn(ctx, mock_struct_path.clone(), ordered, false))
        .chain(
            static_fns
                .iter()
                .map(|ordered| map_fn(ctx, mock_struct_path.clone(), ordered, true)),
        )
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
    pub generics: Generics,
    pub trait_path: Path,
}
pub(crate) struct ResultForTrait {
    pub trait_impl: ItemImpl,
    pub maybe_base_fns_impl: Option<ItemImpl>,
}
pub(crate) fn generate_for_trait(
    ctx: &Context,
    span: Span,
    ParamsForTrait {
        mock_struct_path,
        associated_fns,
        static_fns,
        generics,
        trait_path,
    }: ParamsForTrait,
) -> ResultForTrait {
    let maybe_base_fns_impl = ctx.support_base_calling.then(|| {
        let base_fn_items = associated_fns
            .iter()
            .chain(static_fns.iter())
            .map(|ordered| {
                ordered.clone_map(|x| try_extract_base_fn(span, mock_struct_path.clone(), &x))
            })
            .filter_map(|ordered| match ordered.value {
                Some(x) => Some(Ordered::new(ordered.order_number, x)),
                _ => None,
            })
            .map(|x| ImplItem::Fn(x.value))
            .collect();
        let base_fns_impl = generate_item_impl(
            span,
            generics.clone(),
            mock_struct_path.clone(),
            base_fn_items,
            None,
        );
        return base_fns_impl;
    });
    let mut fns: Vec<_> = associated_fns
        .iter()
        .map(|ordered| map_fn(ctx, mock_struct_path.clone(), ordered, false))
        .chain(
            static_fns
                .iter()
                .map(|ordered| map_fn(ctx, mock_struct_path.clone(), ordered, true)),
        )
        .collect();
    fns.sort_by(|a, b| a.order_number.cmp(&b.order_number));
    let items = fns.into_iter().map(|x| ImplItem::Fn(x.value)).collect();
    let trait_impl = generate_item_impl(span, generics, mock_struct_path, items, Some(trait_path));
    let result = ResultForTrait {
        trait_impl,
        maybe_base_fns_impl,
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
    is_static: bool,
) -> Ordered<ImplItemFn> {
    ordered_fn_info.clone_map(|x| {
        let span = x.spans.inputs;
        ImplItemFn {
            attrs: x.attributes.clone(),
            vis: if is_static {
                Visibility::Inherited
            } else {
                x.visibility.clone()
            },
            modifiers: FnModifiers::default(),
            sig: *x.source_signature.clone(),
            block: if is_static {
                static_fn_block::generate(
                    ctx,
                    span,
                    mock_struct_path,
                    &x,
                    if x.maybe_base_impl.is_some() {
                        BaseFnKind::Associated(base_fn::get_base_fn_ident(&x.fn_ident))
                    } else {
                        BaseFnKind::None
                    },
                )
            } else {
                associated_method_block::generate(
                    ctx,
                    span,
                    mock_struct_path,
                    &x,
                    if x.maybe_base_impl.is_some() {
                        Some(base_fn::get_base_fn_ident(&x.fn_ident))
                    } else {
                        None
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
) -> Option<ImplItemFn> {
    fn_info.maybe_base_impl.clone().map(|base_impl| {
        base_fn::generate_associated(
            span,
            base_fn::AssociatedParams {
                fn_info,
                target_struct_path,
                base_impl,
                maybe_associated_items_info: None,
            },
        )
    })
}
