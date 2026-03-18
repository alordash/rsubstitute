use crate::constants;
use crate::syntax::*;
use proc_macro::TokenStream;
use quote::{format_ident, ToTokens};
use std::cell::LazyCell;
use syn::punctuated::Punctuated;
use syn::*;

pub(crate) fn handle(item: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(item as ItemStruct);

    let trait_path = path::create(constants::I_ARGS_TUPLE_PROVIDER_TRAIT_IDENT.clone());
    let self_ty = Box::new(r#type::create_from_struct(&item_struct));
    let get_arg_infos_fn = generate_get_ptr_to_boxed_tuple_of_refs_fn(&item_struct);
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
    return item_impl.into_token_stream().into();
}

const GET_PTR_TO_BOXED_TUPLE_OF_REFS_FN_SIGNATURE: LazyCell<Signature> = LazyCell::new(|| {
    let signature = Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: Default::default(),
        ident: format_ident!("get_ptr_to_boxed_tuple_of_refs"),
        generics: Generics::default(),
        paren_token: Default::default(),
        inputs: [constants::REF_SELF_ARG.clone()].into_iter().collect(),
        variadic: None,
        output: ReturnType::Type(
            Default::default(),
            Box::new(constants::MUT_VOID_PTR_TYPE.clone()),
        ),
    };
    return signature;
});

fn generate_get_ptr_to_boxed_tuple_of_refs_fn(item_struct: &ItemStruct) -> ImplItem {
    let return_stmt = generate_return_stmt(item_struct);
    let block = Block {
        brace_token: Default::default(),
        stmts: vec![return_stmt],
    };
    let impl_item = ImplItem::Fn(ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        defaultness: None,
        sig: GET_PTR_TO_BOXED_TUPLE_OF_REFS_FN_SIGNATURE.clone(),
        block,
    });
    return impl_item;
}

fn generate_return_stmt(item_struct: &ItemStruct) -> Stmt {
    let fields_exprs: Punctuated<_, Token![,]> = item_struct
        .fields
        .iter()
        .filter(|field| !field::is_phantom_data(field))
        .map(|field| generate_field_expr_ref_expr(field))
        .collect();
    let tuple_expr = Expr::Tuple(ExprTuple {
        attrs: Vec::new(),
        paren_token: Default::default(),
        elems: fields_exprs,
    });
    let box_new_expr = expr_call::create(constants::BOX_NEW_EXPR.clone(), tuple_expr);
    let box_leak_expr = expr_call::create(constants::BOX_LEAK_EXPR.clone(), box_new_expr);
    let as_mut_anonymous_expr = Expr::Cast(ExprCast {
        attrs: Vec::new(),
        expr: Box::new(box_leak_expr),
        as_token: Default::default(),
        ty: Box::new(constants::MUT_INFER_PTR_TYPE.clone()),
    });
    let as_mut_expr = Expr::Cast(ExprCast {
        attrs: Vec::new(),
        expr: Box::new(as_mut_anonymous_expr),
        as_token: Default::default(),
        ty: Box::new(constants::MUT_VOID_PTR_TYPE.clone()),
    });

    let stmt = Stmt::Expr(as_mut_expr, None);
    return stmt;
}

fn generate_field_expr_ref_expr(field: &Field) -> Expr {
    let field_ident = field
        .ident
        .clone()
        .expect("Call struct fields should have ident.");
    let field_access =
        field_access_expr::create(vec![constants::SELF_IDENT.clone(), field_ident.clone()]);
    let field_reference = expr_reference::create(field_access);
    return field_reference;
}
