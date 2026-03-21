use crate::constants;
use crate::syntax::*;
use syn::punctuated::Punctuated;
use syn::*;

pub(crate) fn generate(item_struct: &ItemStruct) -> ItemImpl {
    let trait_path = path::create(constants::CLONE_TRAIT_IDENT.clone());
    let self_ty = Box::new(r#type::create_from_struct(item_struct));
    let get_arg_infos_fn = generate_clone_fn(item_struct);
    let item_impl = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Default::default(),
        generics: generics::remove_default_values(item_struct.generics.clone()),
        trait_: Some((None, trait_path, Default::default())),
        self_ty,
        brace_token: Default::default(),
        items: vec![get_arg_infos_fn],
    };
    return item_impl;
}

fn generate_clone_fn(item_struct: &ItemStruct) -> ImplItem {
    let return_stmt = generate_return_stmt(item_struct);
    let block = Block {
        brace_token: Default::default(),
        stmts: vec![return_stmt],
    };
    let impl_item = ImplItem::Fn(ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        defaultness: None,
        sig: constants::CLONE_FN_SIGNATURE.clone(),
        block,
    });
    return impl_item;
}

fn generate_return_stmt(item_struct: &ItemStruct) -> Stmt {
    let fields: Punctuated<_, Token![,]> = item_struct
        .fields
        .iter()
        .map(|field| generate_field_value(field))
        .collect();

    let self_expr = Expr::Struct(ExprStruct {
        attrs: Vec::new(),
        qself: None,
        path: constants::SELF_TYPE_PATH.clone(),
        brace_token: Default::default(),
        fields,
        dot2_token: None,
        rest: None,
    });
    let stmt = Stmt::Expr(self_expr, None);
    return stmt;
}

fn generate_field_value(field: &Field) -> FieldValue {
    let field_ident = field
        .ident
        .clone()
        .expect("Call struct fields should have ident.");
    let field_clone_expr = expr_method_call::create_with_base_receiver(
        Expr::Paren(ExprParen {
            attrs: Vec::new(),
            paren_token: Default::default(),
            expr: Box::new(expr_reference::create(field_access_expr::create(vec![
                constants::SELF_IDENT.clone(),
                field_ident.clone(),
            ]))),
        }),
        Vec::new(),
        constants::CLONE_FN_IDENT.clone(),
        Vec::new(),
    );
    let field_value = FieldValue {
        attrs: Vec::new(),
        member: Member::Named(field_ident),
        colon_token: Some(Default::default()),
        expr: Expr::MethodCall(field_clone_expr),
    };
    return field_value;
}
