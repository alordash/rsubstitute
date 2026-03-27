use crate::constants;
use crate::syntax::extensions::*;
use crate::syntax::*;
use proc_macro::TokenStream;
use quote::{ToTokens, format_ident};
use std::cell::LazyCell;
use syn::punctuated::Punctuated;
use syn::*;

pub(crate) fn handle(item: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(item as ItemStruct);

    let fmt_args_impl = create_get_received_nothing_else_error_msgs_impl_item(&item_struct);
    let item_impl = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Default::default(),
        generics: generics::remove_default_values(item_struct.generics.clone()),
        trait_: Some((
            None,
            path::create(constants::I_MOCK_DATA_TRAIT_IDENT.clone()),
            Default::default(),
        )),
        self_ty: Box::new(r#type::create_from_struct(&item_struct)),
        brace_token: Default::default(),
        items: vec![fmt_args_impl],
    };
    return item_impl.into_token_stream().into();
}

const GET_UNEXPECTED_CALLS_ERROR_MSGS_FN_IDENT: LazyCell<Ident> =
    LazyCell::new(|| format_ident!("get_unexpected_calls_error_msgs"));

fn create_get_received_nothing_else_error_msgs_impl_item(item_struct: &ItemStruct) -> ImplItem {
    let sig = constants::I_MOCK_DATA_GET_RECEIVED_NOTHING_ELSE_ERROR_MSGS_FN_SIGNATURE.clone();
    let block = create_get_received_nothing_else_error_msgs_block(item_struct);
    let impl_item_fn = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        defaultness: None,
        sig,
        block,
    };
    let impl_item = ImplItem::Fn(impl_item_fn);
    return impl_item;
}

fn create_get_received_nothing_else_error_msgs_block(item_struct: &ItemStruct) -> Block {
    let vec_macro_args: Punctuated<_, Token![,]> = item_struct
        .fields
        .iter()
        .filter(|field| match &field.ty {
            Type::Path(type_path) => {
                let Some(first_segment) = type_path.path.segments.first() else {
                    return false;
                };
                return first_segment.ident == constants::FN_DATA_TYPE_IDENT.clone();
            }
            _ => false,
        })
        .map(|field| {
            method_call::create(
                vec![constants::SELF_IDENT.clone(), field.get_required_ident()],
                GET_UNEXPECTED_CALLS_ERROR_MSGS_FN_IDENT.clone(),
                Vec::new(),
            )
        })
        .collect();
    let return_stmt = Stmt::Expr(
        Expr::Return(ExprReturn {
            attrs: Vec::new(),
            return_token: Default::default(),
            expr: Some(Box::new(Expr::Macro(ExprMacro {
                attrs: Vec::new(),
                mac: Macro {
                    path: constants::MACRO_VEC_PATH.clone(),
                    bang_token: Default::default(),
                    delimiter: MacroDelimiter::Bracket(Default::default()),
                    tokens: vec_macro_args.into_token_stream(),
                },
            }))),
        }),
        Some(Default::default()),
    );
    let block = Block {
        brace_token: Default::default(),
        stmts: vec![return_stmt],
    };
    return block;
}
