use crate::common::generics_field;
use crate::common::models::*;
use crate::generation::base_fn;
use crate::generation::common::*;
use crate::generation::fn_info::models::*;
use crate::generation::mock_controls::models::*;
use crate::generation::mock_struct::common::*;
use crate::generation::mock_struct::models::*;
use crate::generation::mock_struct::*;
use crate::generation::trait_info::models::*;
use crate::preparation::common::models::*;
use crate::preparation::r#trait::models::*;
use crate::syntax::*;
use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::*;

pub(crate) struct Params<'a> {
    pub mock_struct_ident: Ident,
    pub trait_info: &'a TraitInfo,
    pub maybe_associated_controls: Option<AssociatedControls>,
    pub maybe_static_controls: Option<StaticControls>,
}

pub(crate) fn generate(
    ctx: &Context,
    span: Span,
    Params {
        mock_struct_ident,
        trait_info,
        maybe_associated_controls,
        maybe_static_controls,
    }: Params,
) -> TraitMockStruct {
    let path =
        path::from_ident_with_generics(mock_struct_ident.clone(), &trait_info.merged_generics);
    let item_struct = ItemStruct {
        attrs: Vec::new(),
        vis: Visibility::Public(Token![pub](span)),
        struct_token: Token![struct](span),
        ident: mock_struct_ident,
        generics: trait_info.merged_generics.clone(),
        fields: Fields::Named(FieldsNamed {
            brace_token: token::Brace(span),
            named: punctuated([
                generics_field::new_field(span, trait_info.merged_generics.clone(), None),
                data_field::new_field(span, data_field::Params { public: true }),
            ]),
        }),
        semi_token: None,
    };
    let clone_impl = clone_impl::generate(
        span,
        item_struct.generics.clone(),
        path.clone(),
        &item_struct.fields,
    );
    let trait_impl = generate_trait_impl(ctx, span, trait_info, path.clone());
    let inner_impl = generate_inner_impl(
        ctx,
        span,
        trait_info,
        path.clone(),
        &maybe_associated_controls,
        &maybe_static_controls,
    );

    let result = TraitMockStruct {
        path,
        item_struct,
        clone_impl,
        trait_impl,
        inner_impl,
        maybe_associated_controls,
        maybe_static_controls,
    };
    return result;
}

fn generate_trait_impl(
    ctx: &Context,
    span: Span,
    trait_info: &TraitInfo,
    mock_struct_path: Path,
) -> ItemImpl {
    let mut items_with_order: Vec<_> = trait_info
        .constants
        .iter()
        .map(map_const)
        .chain(trait_info.assoc_types.iter().map(map_assoc_type))
        .chain(
            trait_info
                .associated_fns
                .iter()
                .map(|x| map_method(ctx, mock_struct_path.clone(), x)),
        )
        .chain(
            trait_info
                .static_fns
                .iter()
                .map(|x| map_fn(ctx, mock_struct_path.clone(), x)),
        )
        .collect();
    items_with_order.sort_by(|a, b| a.order_number.cmp(&b.order_number));

    let items = items_with_order.into_iter().map(|x| x.value).collect();

    let result = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: trait_info.unsafety,
        impl_token: Token![impl](span),
        generics: trait_info.merged_generics.clone(),
        trait_: Some((None, trait_info.path.clone(), Token![for](span))),
        self_ty: Box::new(Type::Path(TypePath {
            qself: None,
            path: mock_struct_path,
        })),
        brace_token: token::Brace(span),
        items,
    };
    return result;
}

fn map_const(ordered_const: &Ordered<TraitItemConstSyntax>) -> Ordered<ImplItem> {
    ordered_const.clone_map(|x| {
        let span = x.corresponding_generic_param_path.span();
        ImplItem::Const(ImplItemConst {
            attrs: x.item.attrs.clone(),
            vis: Visibility::Inherited,
            defaultness: None,
            const_token: x.item.const_token.clone(),
            ident: x.item.ident.clone(),
            generics: x.item.generics.clone(),
            colon_token: x.item.colon_token.clone(),
            ty: x.item.ty.clone(),
            eq_token: Token![=](span),
            expr: Expr::Path(ExprPath {
                attrs: Vec::new(),
                qself: None,
                path: x.corresponding_generic_param_path.clone(),
            }),
            semi_token: Token![;](span),
        })
    })
}

fn map_assoc_type(ordered_assoc_type: &Ordered<TraitItemTypeSyntax>) -> Ordered<ImplItem> {
    ordered_assoc_type.clone_map(|x| {
        let span = x.corresponding_generic_param_path.span();
        ImplItem::Type(ImplItemType {
            attrs: x.item.attrs.clone(),
            vis: Visibility::Inherited,
            defaultness: None,
            type_token: x.item.type_token.clone(),
            ident: x.item.ident.clone(),
            generics: x.item.generics.clone(),
            eq_token: Token![=](span),
            ty: Type::Path(TypePath {
                qself: None,
                path: x.corresponding_generic_param_path.clone(),
            }),
            semi_token: Token![;](span),
        })
    })
}

fn map_method(
    ctx: &Context,
    mock_struct_path: Path,
    ordered_method: &Ordered<FnInfo>,
) -> Ordered<ImplItem> {
    ordered_method.clone_map(|x| {
        let span = x.spans.inputs;
        ImplItem::Fn(ImplItemFn {
            attrs: x.attributes.clone(),
            vis: Visibility::Inherited,
            defaultness: None, // TODO - verify that it's always None, IIRC you can trait Trait { default fn f() {} }
            sig: *x.source_signature.clone(),
            block: associated_method_block::generate(
                ctx,
                span,
                mock_struct_path,
                x,
                if x.maybe_base_impl.is_some() {
                    Some(base_fn::get_base_fn_ident(&x.fn_ident))
                } else {
                    None
                },
            ),
        })
    })
}

fn map_fn(
    ctx: &Context,
    mock_struct_path: Path,
    ordered_method: &Ordered<FnInfo>,
) -> Ordered<ImplItem> {
    ordered_method.clone_map(|x| {
        let span = x.spans.inputs;
        ImplItem::Fn(ImplItemFn {
            attrs: x.attributes.clone(),
            vis: Visibility::Inherited,
            defaultness: None, // TODO - verify that it's always None, IIRC you can trait Trait { default fn f() {} }
            sig: *x.source_signature.clone(),
            block: static_fn_block::generate(
                ctx,
                span,
                mock_struct_path,
                x,
                if x.maybe_base_impl.is_some() {
                    BaseFnKind::Associated(base_fn::get_base_fn_ident(&x.fn_ident))
                } else {
                    BaseFnKind::None
                },
            ),
        })
    })
}

fn generate_inner_impl(
    ctx: &Context,
    span: Span,
    trait_info: &TraitInfo,
    mock_struct_path: Path,
    maybe_associated_controls: &Option<AssociatedControls>,
    maybe_static_controls: &Option<StaticControls>,
) -> ItemImpl {
    let mock_struct_fn_new = mock_struct_fn_new::new(span);
    let associated_controls_creation_fns =
        maybe_associated_controls
            .as_ref()
            .map(|associated_controls| {
                [
                    control_creation_fn::generate_associated(
                        span,
                        associated_controls.setup_struct.path.clone(),
                        ControlType::Setup,
                    ),
                    control_creation_fn::generate_associated(
                        span,
                        associated_controls.received_struct.path.clone(),
                        ControlType::Received,
                    ),
                ]
            });
    let static_controls_creation_fns = maybe_static_controls.as_ref().map(|static_controls| {
        [
            control_creation_fn::generate_static(
                span,
                static_controls.static_setup_struct.path.clone(),
                StaticControlType::Setup {
                    mock_generic_argument: GenericArgument::Type(Type::Path(TypePath {
                        qself: None,
                        path: mock_struct_path.clone(),
                    })),
                },
            ),
            control_creation_fn::generate_static(
                span,
                static_controls.static_received_struct.path.clone(),
                StaticControlType::Received,
            ),
        ]
    });
    let base_fns = if ctx.support_base_calling {
        Some(
            trait_info
                .associated_fns
                .iter()
                .filter_map(|fn_info| {
                    try_extract_base_fn(span, &mock_struct_path, trait_info, fn_info, false)
                })
                .chain(trait_info.static_fns.iter().filter_map(|fn_info| {
                    try_extract_base_fn(span, &mock_struct_path, trait_info, fn_info, true)
                })),
        )
    } else {
        None
    };
    let items = core::iter::once(mock_struct_fn_new)
        .chain(associated_controls_creation_fns.into_iter().flatten())
        .chain(static_controls_creation_fns.into_iter().flatten())
        .chain(base_fns.into_iter().flatten())
        .map(ImplItem::Fn)
        .collect();

    let result = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Token![impl](span),
        generics: trait_info.merged_generics.clone(),
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

fn try_extract_base_fn(
    span: Span,
    mock_struct_path: &Path,
    trait_info: &TraitInfo,
    fn_info: &FnInfo,
    is_static: bool,
) -> Option<ImplItemFn> {
    fn_info.maybe_base_impl.clone().map(|base_impl| {
        base_fn::generate_associated(
            span,
            base_fn::AssociatedParams {
                fn_info,
                target_struct_path: mock_struct_path.clone(),
                base_impl,
                maybe_associated_items_info: Some(&trait_info.associated_items_info),
                is_static,
            },
        )
    })
}
