use crate::constants;
use crate::syntax::*;
use proc_macro::TokenStream;
use quote::{format_ident, ToTokens};
use std::cell::LazyCell;
use syn::punctuated::Punctuated;
use syn::*;

pub(crate) trait IDeriveMockDataMacroHandler {
    fn handle(&self, item: proc_macro::TokenStream) -> proc_macro::TokenStream;
}

pub(crate) struct DeriveMockDataMacroHandler;

impl IDeriveMockDataMacroHandler for DeriveMockDataMacroHandler {
    fn handle(&self, item: TokenStream) -> TokenStream {
        let item_struct = parse_macro_input!(item as ItemStruct);

        let fmt_args_impl =
            self.create_get_received_nothing_else_error_msgs_impl_item(&item_struct);
        let item_impl = ItemImpl {
            attrs: Vec::new(),
            defaultness: None,
            unsafety: None,
            impl_token: Default::default(),
            generics: item_struct.generics.clone(),
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
}

impl DeriveMockDataMacroHandler {
    const GET_UNEXPECTED_CALLS_ERROR_MSGS_FN_IDENT: LazyCell<Ident> =
        LazyCell::new(|| format_ident!("get_unexpected_calls_error_msgs"));

    fn create_get_received_nothing_else_error_msgs_impl_item(
        &self,
        item_struct: &ItemStruct,
    ) -> ImplItem {
        let sig = constants::I_MOCK_DATA_GET_RECEIVED_NOTHING_ELSE_ERROR_MSGS_FN_SIGNATURE.clone();
        let block = self.create_get_received_nothing_else_error_msgs_block(item_struct);
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

    fn create_get_received_nothing_else_error_msgs_block(&self, item_struct: &ItemStruct) -> Block {
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
                expr_method_call::create(
                    vec![constants::SELF_IDENT.clone(), field.get_required_ident()],
                    Self::GET_UNEXPECTED_CALLS_ERROR_MSGS_FN_IDENT.clone(),
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
}
