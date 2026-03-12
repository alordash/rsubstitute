use crate::constants;
use crate::mock_generation::mock_parts_generation::models::*;
use crate::syntax::*;
use syn::*;

pub(crate) fn generate(
    mock_struct: &MockStruct,
    inner_data_struct: &InnerDataStruct,
) -> InnerDataDerefImpl {
    let self_ty = mock_struct.ty.clone();
    let inner_data_type = inner_data_struct.ty.clone();
    let target_type_item = generate_target_type_item(inner_data_type.clone());
    let deref_fn_item = generate_deref_fn_item();
    let items = vec![target_type_item, deref_fn_item];
    let item_impl = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Default::default(),
        generics: mock_struct.item_struct.generics.clone(),
        trait_: Some((
            None,
            constants::DEREF_TRAIT_PATH.clone(),
            Default::default(),
        )),
        self_ty: Box::new(self_ty),
        brace_token: Default::default(),
        items,
    };
    let inner_data_deref_impl = InnerDataDerefImpl { item_impl };
    return inner_data_deref_impl;
}

fn generate_target_type_item(ty: Type) -> ImplItem {
    let target_type_item = ImplItemType {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        defaultness: None,
        type_token: Default::default(),
        ident: constants::DEREF_TARGET_TYPE_IDENT.clone(),
        generics: Generics::default(),
        eq_token: Default::default(),
        ty,
        semi_token: Default::default(),
    };
    return ImplItem::Type(target_type_item);
}

fn generate_deref_fn_item() -> ImplItem {
    let sig = Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: Default::default(),
        ident: constants::DEREF_FN_IDENT.clone(),
        generics: Generics::default(),
        paren_token: Default::default(),
        inputs: [constants::REF_SELF_ARG.clone()].into_iter().collect(),
        variadic: None,
        output: ReturnType::Type(
            Default::default(),
            Box::new(r#type::reference(
                Type::Path(TypePath {
                    qself: None,
                    path: path::create_from_parts(vec![
                        constants::SELF_TYPE_IDENT.clone(),
                        constants::DEREF_TARGET_TYPE_IDENT.clone(),
                    ]),
                }),
                None,
            )),
        ),
    };
    let block = Block {
        brace_token: Default::default(),
        stmts: vec![Stmt::Expr(
            expr_reference::create(field_access_expr::create(vec![
                constants::SELF_IDENT.clone(),
                constants::INNER_DATA_FIELD_IDENT.clone(),
            ])),
            None,
        )],
    };
    let deref_fn_item = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        defaultness: None,
        sig,
        block,
    };
    return ImplItem::Fn(deref_fn_item);
}
