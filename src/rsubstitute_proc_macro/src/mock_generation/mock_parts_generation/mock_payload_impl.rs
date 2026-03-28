use crate::mock_generation::fn_info_generation::models::*;
use crate::mock_generation::mock_parts_generation::models::*;
use crate::mock_generation::mock_parts_generation::*;
use crate::syntax::*;
use proc_macro2::Ident;
use syn::*;

pub(crate) fn generate(
    trait_ident: Ident,
    mock_type: &MockType,
    fn_infos: &[FnInfo],
    associated_generics: AssociatedGenerics,
) -> MockPayloadImpl {
    let trait_path =
        path::create_with_generics(trait_ident, mock_type.generics.source_generics.clone());
    let rest_impl_items = convert_associated_generics_to_impl_items(associated_generics);

    let mock_impl = generate_core(
        Vec::new(),
        mock_type.ty_path.clone(),
        mock_type
            .generics
            .impl_generics_without_default_values
            .clone(),
        fn_infos,
        Some(trait_path),
        None,
        rest_impl_items,
        0,
    );
    return mock_impl;
}

pub(crate) fn generate_for_struct_trait(
    trait_path: Path,
    fn_infos: &[FnInfo],
    containing_trait_ident: &Ident,
    trait_generics: &Generics,
    trait_self_ty_path: TypePath,
    default_self_ty_path_generics_arguments_len: usize,
    rest_impl_items: Vec<ImplItem>,
) -> MockPayloadImpl {
    let mock_impl = generate_core(
        Vec::new(),
        trait_self_ty_path,
        trait_generics.clone(),
        fn_infos,
        Some(trait_path),
        Some(containing_trait_ident),
        rest_impl_items,
        default_self_ty_path_generics_arguments_len,
    );
    return mock_impl;
}

pub(crate) fn generate_for_struct(
    attrs: Vec<Attribute>,
    mock_type: &MockType,
    fn_infos: &[FnInfo],
) -> MockPayloadImpl {
    let mock_impl = generate_core(
        attrs,
        mock_type.ty_path.clone(),
        mock_type
            .generics
            .impl_generics_without_default_values
            .clone(),
        fn_infos,
        None,
        None,
        Vec::new(),
        0,
    );
    return mock_impl;
}

fn generate_core(
    attrs: Vec<Attribute>,
    self_ty_path: TypePath,
    generics: Generics,
    fn_infos: &[FnInfo],
    maybe_trait_path: Option<Path>,
    maybe_containing_trait_ident: Option<&Ident>,
    rest_impl_items: Vec<ImplItem>,
    default_self_ty_path_generics_arguments_len: usize,
) -> MockPayloadImpl {
    let items = rest_impl_items
        .into_iter()
        .chain(
            fn_infos
                .iter()
                .map(|x| {
                    generate_impl_item_fn(
                        x,
                        &self_ty_path,
                        maybe_containing_trait_ident,
                        default_self_ty_path_generics_arguments_len,
                    )
                })
                .map(ImplItem::Fn),
        )
        .collect();
    let trait_ = maybe_trait_path.map(|trait_path| (None, trait_path, Default::default()));

    let item_impl = ItemImpl {
        attrs,
        defaultness: None,
        unsafety: None,
        impl_token: Default::default(),
        generics,
        trait_,
        self_ty: Box::new(Type::Path(self_ty_path)),
        brace_token: Default::default(),
        items,
    };
    let mock_impl = MockPayloadImpl { item_impl };
    return mock_impl;
}

fn generate_impl_item_fn(
    fn_info: &FnInfo,
    self_ty_path: &TypePath,
    maybe_containing_trait_ident: Option<&Ident>,
    default_self_ty_path_generics_arguments_len: usize,
) -> ImplItemFn {
    let sig = Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: Default::default(),
        ident: fn_info.parent.fn_ident.clone(),
        generics: fn_info.parent.sig_generics.clone(),
        paren_token: Default::default(),
        inputs: mock_fn_inputs::generate(&fn_info.parent.arguments),
        variadic: None,
        output: fn_info.parent.return_value.clone(),
    };
    let block = match maybe_containing_trait_ident {
        None => mock_fn_block::generate_for_trait(fn_info),
        Some(containing_trait_ident) => mock_fn_block::generate_for_struct_trait_fn(
            fn_info,
            containing_trait_ident,
            self_ty_path.clone(),
            default_self_ty_path_generics_arguments_len,
        ),
    };
    let impl_item_fn = ImplItemFn {
        attrs: fn_info.parent.attrs.clone(),
        vis: fn_info.parent.visibility.clone(),
        defaultness: None,
        sig,
        block,
    };
    return impl_item_fn;
}

fn convert_associated_generics_to_impl_items(
    associated_generics: AssociatedGenerics,
) -> Vec<ImplItem> {
    let impl_items = associated_generics
        .trait_items
        .into_iter()
        .filter_map(|trait_item| match trait_item {
            TraitItem::Const(x) => Some(ImplItem::Const(ImplItemConst {
                attrs: x.attrs,
                vis: Visibility::Inherited,
                defaultness: None,
                colon_token: x.colon_token,
                ident: x.ident.clone(),
                generics: x.generics,
                const_token: Default::default(),
                ty: x.ty,
                eq_token: Default::default(),
                expr: path::create_expr(associated_generics::format_associated_param_ident(
                    &associated_generics.parent_ident,
                    &x.ident,
                )),
                semi_token: Default::default(),
            })),
            TraitItem::Type(x) => Some(ImplItem::Type(ImplItemType {
                attrs: x.attrs,
                vis: Visibility::Inherited,
                defaultness: None,
                type_token: x.type_token,
                ident: x.ident.clone(),
                generics: x.generics,
                eq_token: Default::default(),
                ty: r#type::create(associated_generics::format_associated_param_ident(
                    &associated_generics.parent_ident,
                    &x.ident,
                )),
                semi_token: Default::default(),
            })),
            _ => None,
        })
        .collect();
    return impl_items;
}
