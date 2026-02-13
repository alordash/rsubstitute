use crate::constants;
use crate::syntax::*;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{format_ident, ToTokens};
use std::cell::LazyCell;
use std::sync::Arc;
use syn::punctuated::Punctuated;
use syn::token::Bracket;
use syn::*;

pub trait IDeriveArgsInfosProviderMacroHandler {
    fn handle(&self, item: proc_macro::TokenStream) -> proc_macro::TokenStream;
}

pub struct DeriveArgsInfosProviderMacroHandler {
    pub(crate) path_factory: Arc<dyn IPathFactory>,
    pub(crate) type_factory: Arc<dyn ITypeFactory>,
    pub(crate) expr_method_call_factory: Arc<dyn IExprMethodCallFactory>,
}

impl IDeriveArgsInfosProviderMacroHandler for DeriveArgsInfosProviderMacroHandler {
    fn handle(&self, item: TokenStream) -> TokenStream {
        let item_struct = parse_macro_input!(item as ItemStruct);

        let trait_path = self
            .path_factory
            .create(constants::I_ARG_INFOS_PROVIDER_TRAIT_IDENT.clone());
        let self_ty = Box::new(self.type_factory.create_from_struct(&item_struct));
        let get_arg_infos_fn = self.generate_get_arg_infos_fn(&item_struct);
        let item_impl = ItemImpl {
            attrs: Vec::new(),
            defaultness: None,
            unsafety: None,
            impl_token: Default::default(),
            generics: item_struct.generics.clone(),
            trait_: Some((None, trait_path, Default::default())),
            self_ty,
            brace_token: Default::default(),
            items: vec![get_arg_infos_fn],
        };
        return item_impl.into_token_stream().into();
    }
}

impl DeriveArgsInfosProviderMacroHandler {
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

    fn generate_get_arg_infos_fn(&self, item_struct: &ItemStruct) -> ImplItem {
        let return_stmt = self.generate_return_stmt(item_struct);
        let block = Block {
            brace_token: Default::default(),
            stmts: vec![return_stmt],
        };
        let impl_item = ImplItem::Fn(ImplItemFn {
            attrs: Vec::new(),
            vis: Visibility::Inherited,
            defaultness: None,
            sig: Self::GET_ARG_INFOS_FN_SIGNATURE.clone(),
            block,
        });
        return impl_item;
    }

    fn generate_return_stmt(&self, item_struct: &ItemStruct) -> Stmt {
        let check_exprs: Punctuated<_, Token![,]> = item_struct
            .fields
            .iter()
            .map(|field| self.generate_arg_info_new_expr(field))
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

    fn generate_arg_info_new_expr(&self, field: &Field) -> Expr {
        let field_ident = field
            .ident
            .clone()
            .expect("Call struct fields should have ident.");
        let field_name_arg = Expr::Lit(ExprLit {
            attrs: Vec::new(),
            lit: Lit::Str(LitStr::new(&field_ident.to_string(), Span::call_site())),
        });
        let field_access_arg = Expr::MethodCall(self.expr_method_call_factory.create(
            vec![constants::SELF_IDENT.clone(), field_ident],
            constants::CLONE_FN_IDENT.clone(),
            Vec::new(),
        ));

        let expr = Expr::Call(ExprCall {
            attrs: Vec::new(),
            func: Box::new(self.path_factory.create_expr_from_parts(vec![
                Self::ARG_INFO_TYPE_IDENT.clone(),
                constants::NEW_IDENT.clone(),
            ])),
            paren_token: Default::default(),
            args: [field_name_arg, field_access_arg].into_iter().collect(),
        });
        return expr;
    }
}
