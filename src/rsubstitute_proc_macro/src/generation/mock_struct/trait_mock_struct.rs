use crate::common::generics_field;
use crate::common::models::Context;
use crate::generation::fn_info::models::*;
use crate::generation::mock_struct::associated_fn_block;
use crate::generation::mock_struct::models::*;
use crate::generation::trait_info::models::*;
use crate::preparation::common::models::*;
use crate::preparation::r#trait::models::*;
use crate::syntax::*;
use proc_macro2::Span;
use quote::format_ident;
use syn::spanned::Spanned;
use syn::*;

pub(crate) fn generate(ctx: &Context, span: Span, trait_info: &TraitInfo) -> TraitMockStruct {
    let item_struct = ItemStruct {
        attrs: Vec::new(),
        vis: Visibility::Public(Token![pub](span)),
        struct_token: Token![struct](span),
        ident: format_ident!("{}Mock", trait_info.ident),
        generics: trait_info.merged_generics.clone(),
        fields: Fields::Named(FieldsNamed {
            brace_token: token::Brace(span),
            named: punctuated([generics_field::new_field(
                span,
                trait_info.merged_generics.clone(),
                None,
            )]),
        }),
        semi_token: None,
    };
    let path = path::from_ident_with_generics(item_struct.ident.clone(), &item_struct.generics);
    let trait_impl = generate_trait_impl(ctx, span, trait_info, path.clone());

    let result = TraitMockStruct {
        path,
        item_struct,
        trait_impl,
        inner_impl: todo!(),
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
                .methods
                .iter()
                .map(|x| map_method(ctx, mock_struct_path.clone(), x)),
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
            block: associated_fn_block::generate(
                ctx,
                span,
                mock_struct_path,
                x,
                if x.maybe_base_impl.is_some() {
                    Some(x.fn_ident.clone())
                } else {
                    None
                },
            ),
        })
    })
}
