use crate::constants;
use crate::syntax::*;
use proc_macro::TokenStream;
use proc_macro2::Literal;
use quote::{ToTokens, quote};
use std::sync::Arc;
use syn::token::Paren;
use syn::*;

pub trait IDeriveArgsFormatterMacroHandler {
    fn handle(&self, item: proc_macro::TokenStream) -> proc_macro::TokenStream;
}

pub(crate) struct DeriveArgsFormatterMacroHandler {
    pub type_factory: Arc<dyn ITypeFactory>,
    pub field_access_expr_factory: Arc<dyn IFieldAccessExprFactory>,
    pub field_checker: Arc<dyn IFieldChecker>,
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
                constants::I_ARGS_FORMATTER_TRAIT_PATH.clone(),
                Default::default(),
            )),
            self_ty: Box::new(self.type_factory.create_from_struct(&item_struct)),
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
            .skip_while(|field| self.field_checker.is_phantom_data(field))
            .map(|_| "{:?}")
            .collect::<Vec<_>>()
            .join(", ");
        let literal = Literal::string(&literal_str);
        let args: Vec<_> = item_struct
            .fields
            .iter()
            .skip_while(|field| self.field_checker.is_phantom_data(field))
            .map(|field| {
                self.field_access_expr_factory.create(vec![
                    constants::SELF_IDENT.clone(),
                    field.get_required_ident(),
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
