use crate::constants;
use crate::syntax::{IFieldAccessExprFactory, IPathFactory, ITypeFactory};
use proc_macro::TokenStream;
use proc_macro2::Literal;
use quote::{ToTokens, quote};
use std::rc::Rc;
use syn::token::Paren;
use syn::*;

pub trait IDeriveArgsFormatterMacroHandler {
    fn handle(&self, item: proc_macro::TokenStream) -> proc_macro::TokenStream;
}

pub(crate) struct DeriveArgsFormatterMacroHandler {
    pub path_factory: Rc<dyn IPathFactory>,
    pub type_factory: Rc<dyn ITypeFactory>,
    pub field_access_expr_factory: Rc<dyn IFieldAccessExprFactory>,
}

impl IDeriveArgsFormatterMacroHandler for DeriveArgsFormatterMacroHandler {
    fn handle(&self, item: TokenStream) -> TokenStream {
        let item_struct = parse_macro_input!(item as ItemStruct);

        let fmt_args_impl = self.create_fmt_args_impl_item(&item_struct);
        let item_impl = ItemImpl {
            attrs: Vec::new(),
            defaultness: None,
            unsafety: None,
            impl_token: Default::default(),
            generics: item_struct.generics.clone(),
            trait_: Some((
                None,
                self.path_factory
                    .create(constants::I_ARGS_FORMATTER_TRAIT_IDENT.clone()),
                Default::default(),
            )),
            self_ty: Box::new(
                self.type_factory
                    .create_with_generics(item_struct.ident.clone(), item_struct.generics.clone()),
            ),
            brace_token: Default::default(),
            items: vec![fmt_args_impl],
        };
        return item_impl.into_token_stream().into();
    }
}

impl DeriveArgsFormatterMacroHandler {
    fn create_fmt_args_impl_item(&self, item_struct: &ItemStruct) -> ImplItem {
        let sig = Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: Default::default(),
            ident: constants::I_ARGS_FORMATTER_FN_IDENT.clone(),
            generics: Generics::default(),
            paren_token: Default::default(),
            inputs: [constants::REF_SELF_ARG.clone()].into_iter().collect(),
            variadic: None,
            output: ReturnType::Type(Default::default(), Box::new(constants::STRING_TYPE.clone())),
        };
        let block = self.create_fmt_args_block(item_struct);
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

    fn create_fmt_args_block(&self, item_struct: &ItemStruct) -> Block {
        let literal_str = item_struct
            .fields
            .iter()
            .skip(1)
            .map(|_| "{:?}")
            .collect::<Vec<_>>()
            .join(", ");
        let literal = Literal::string(&literal_str);
        let args: Vec<_> = item_struct
            .fields
            .iter()
            .skip(1)
            .map(|field| {
                self.field_access_expr_factory.create(&[
                    constants::SELF_IDENT.clone(),
                    field.ident.clone().expect("TODO"),
                ])
            })
            .collect();
        let tokens = quote! { #literal, #(#args),* };
        let format_stmt = Stmt::Expr(
            Expr::Macro(ExprMacro {
                attrs: Vec::new(),
                mac: Macro {
                    path: constants::MACRO_FORMAT_PATH.clone(),
                    bang_token: Default::default(),
                    delimiter: MacroDelimiter::Paren(Paren::default()),
                    tokens,
                },
            }),
            None,
        );
        let block = Block {
            brace_token: Default::default(),
            stmts: vec![format_stmt],
        };
        return block;
    }
}
