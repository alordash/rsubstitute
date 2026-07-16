use crate::common::models::*;
use crate::generation::fn_info::models::*;
use crate::generation::impl_struct_info::models::*;
use crate::generation::mock_struct::models::*;
use crate::generation::mock_struct::*;
use crate::generation::*;
use crate::preparation::models::*;
use proc_macro2::Span;
use syn::*;

pub(crate) fn generate(
    ctx: &Context,
    span: Span,
    mock_struct_path: Path,
    impl_struct_info: &ImplStructInfo,
) -> ItemImpl {
    let base_fns = if ctx.support_base_calling {
        impl_struct_info
            .associated_fns
            .iter()
            .map(|ordered| {
                ordered
                    .clone_map(|x| try_extract_base_fn(span, mock_struct_path.clone(), &x, false))
            })
            .chain(impl_struct_info.static_fns.iter().map(|ordered| {
                ordered.clone_map(|x| try_extract_base_fn(span, mock_struct_path.clone(), &x, true))
            }))
            .filter_map(|ordered| match ordered.value {
                Some(x) => Some(Ordered::new(ordered.order_number, x)),
                _ => None,
            })
            .collect()
    } else {
        Vec::new()
    };
    let mut fns: Vec<_> = impl_struct_info
        .associated_fns
        .iter()
        .map(|ordered| map_fn(ctx, mock_struct_path.clone(), ordered, false))
        .chain(
            impl_struct_info
                .static_fns
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
    let result = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Token![impl](span),
        generics: impl_struct_info.generics.clone(),
        trait_: None,
        self_ty: Box::new(Type::Path(TypePath {
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
            vis: Visibility::Inherited,
            defaultness: None, // TODO - verify that it's always None, IIRC you can trait Trait { default fn f() {} }
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
    is_static: bool,
) -> Option<ImplItemFn> {
    fn_info.maybe_base_impl.clone().map(|base_impl| {
        base_fn::generate_associated(
            span,
            base_fn::AssociatedParams {
                fn_info,
                target_struct_path,
                base_impl,
                maybe_associated_items_info: None,
                is_static,
            },
        )
    })
}
