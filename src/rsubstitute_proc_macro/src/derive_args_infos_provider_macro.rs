use crate::constants;
use crate::mock_generation::mock_parts_generation::*;
use crate::syntax::*;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, ToTokens};
use std::cell::LazyCell;
use syn::punctuated::Punctuated;
use syn::token::Bracket;
use syn::*;

pub(crate) fn handle(item: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(item as ItemStruct);

    let trait_path = path::create(constants::I_ARGS_INFOS_PROVIDER_TRAIT_IDENT.clone());
    let self_ty = Box::new(r#type::create_from_struct(&item_struct));
    let get_arg_infos_fn = generate_get_arg_infos_fn(&item_struct);
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

const GET_ARG_INFOS_FN_SIGNATURE: LazyCell<Signature> = LazyCell::new(|| {
    let signature = Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: Default::default(),
        ident: format_ident!("get_arg_infos"),
        generics: Generics::default(),
        paren_token: Default::default(),
        inputs: [constants::REF_SELF_ARG.clone()].into_iter().collect(),
        variadic: None,
        output: ReturnType::Type(
            Default::default(),
            Box::new(constants::VEC_OF_ARG_INFO_RESULT_TYPE.clone()),
        ),
    };
    return signature;
});

const ARG_INFO_TYPE_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("ArgInfo"));

fn generate_get_arg_infos_fn(item_struct: &ItemStruct) -> ImplItem {
    let return_stmt = generate_return_stmt(item_struct);
    let block = Block {
        brace_token: Default::default(),
        stmts: vec![return_stmt],
    };
    let impl_item = ImplItem::Fn(ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        defaultness: None,
        sig: GET_ARG_INFOS_FN_SIGNATURE.clone(),
        block,
    });
    return impl_item;
}

fn generate_return_stmt(item_struct: &ItemStruct) -> Stmt {
    let check_exprs: Punctuated<_, Token![,]> = item_struct
        .fields
        .iter()
        .filter(|field| !field::is_phantom_data(field))
        .map(|field| generate_arg_info_new_expr(field))
        .collect();
    let vec_expr = Expr::Macro(ExprMacro {
        attrs: Vec::new(),
        mac: Macro {
            path: constants::MACRO_VEC_PATH.clone(),
            bang_token: Default::default(),
            delimiter: MacroDelimiter::Bracket(Bracket::default()),
            tokens: check_exprs.into_token_stream(),
        },
    });
    let stmt = Stmt::Expr(vec_expr, None);
    return stmt;
}

fn generate_arg_info_new_expr(field: &Field) -> Expr {
    let field_ident = field
        .ident
        .clone()
        .expect("Call struct fields should have ident.");
    let field_name_arg = str_lit::create_from_ident(&field_ident);
    let field_value_arg = expr_reference::create(field_access_expr::create(vec![
        constants::SELF_IDENT.clone(),
        field_ident.clone(),
    ]));
    let field_debug_string_arg = debug_string_expr::generate(field_access_expr::create(vec![
        constants::SELF_IDENT.clone(),
        field_ident,
    ]));

    let expr = expr_call::create_with_args(
        path::create_expr_from_parts(vec![
            ARG_INFO_TYPE_IDENT.clone(),
            constants::NEW_IDENT.clone(),
        ]),
        vec![field_name_arg, field_value_arg, field_debug_string_arg],
    );
    return expr;
}
