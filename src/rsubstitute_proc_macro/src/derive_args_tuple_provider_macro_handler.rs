use crate::constants;
use crate::syntax::*;
use proc_macro::TokenStream;
use quote::{ToTokens, format_ident};
use std::cell::LazyCell;
use std::sync::Arc;
use syn::punctuated::Punctuated;
use syn::*;

pub trait IDeriveArgsTupleProviderMacroHandler {
    fn handle(&self, item: proc_macro::TokenStream) -> proc_macro::TokenStream;
}

pub(crate) struct DeriveArgsTupleProviderMacroHandler {
    pub path_factory: Arc<dyn IPathFactory>,
    pub type_factory: Arc<dyn ITypeFactory>,
    pub field_access_expr_factory: Arc<dyn IFieldAccessExprFactory>,
    pub expr_reference_factory: Arc<dyn IExprReferenceFactory>,
    pub field_checker: Arc<dyn IFieldChecker>,
}

impl IDeriveArgsTupleProviderMacroHandler for DeriveArgsTupleProviderMacroHandler {
    fn handle(&self, item: TokenStream) -> TokenStream {
        let item_struct = parse_macro_input!(item as ItemStruct);

        let trait_path = self
            .path_factory
            .create(constants::I_ARGS_TUPLE_PROVIDER_TRAIT_IDENT.clone());
        let self_ty = Box::new(self.type_factory.create_from_struct(&item_struct));
        let get_arg_infos_fn = self.generate_provide_ptr_to_tuple_of_refs_fn(&item_struct);
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

impl DeriveArgsTupleProviderMacroHandler {
    const PROVIDE_PTR_TO_TUPLE_OF_REFS_FN_SIGNATURE: LazyCell<Signature> = LazyCell::new(|| {
        let signature = Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: Default::default(),
            ident: format_ident!("provide_ptr_to_tuple_of_refs"),
            generics: Generics::default(),
            paren_token: Default::default(),
            inputs: [constants::REF_SELF_ARG.clone()].into_iter().collect(),
            variadic: None,
            output: ReturnType::Type(
                Default::default(),
                Box::new(constants::VOID_PTR_TYPE.clone()),
            ),
        };
        return signature;
    });

    fn generate_provide_ptr_to_tuple_of_refs_fn(&self, item_struct: &ItemStruct) -> ImplItem {
        let return_stmt = self.generate_return_stmt(item_struct);
        let block = Block {
            brace_token: Default::default(),
            stmts: vec![return_stmt],
        };
        let impl_item = ImplItem::Fn(ImplItemFn {
            attrs: Vec::new(),
            vis: Visibility::Inherited,
            defaultness: None,
            sig: Self::PROVIDE_PTR_TO_TUPLE_OF_REFS_FN_SIGNATURE.clone(),
            block,
        });
        return impl_item;
    }

    fn generate_return_stmt(&self, item_struct: &ItemStruct) -> Stmt {
        let fields_exprs: Punctuated<_, Token![,]> = item_struct
            .fields
            .iter()
            .filter(|field| !self.field_checker.is_phantom_data(field))
            .map(|field| self.generate_field_expr_ref_expr(field))
            .collect();

        let ptr_expr = Expr::Call(ExprCall {
            attrs: Vec::new(),
            func: Box::new(self.path_factory.create_expr_from_parts(vec![
                format_ident!("core"),
                format_ident!("ptr"),
                format_ident!("from_ref"),
            ])),
            paren_token: Default::default(),
            args: [self.expr_reference_factory.create(Expr::Tuple(ExprTuple {
                attrs: Vec::new(),
                paren_token: Default::default(),
                elems: fields_exprs,
            }))]
            .into_iter()
            .collect(),
        });

        let ptr_cast_expr = Expr::Cast(ExprCast {
            attrs: Vec::new(),
            expr: Box::new(ptr_expr),
            as_token: Default::default(),
            ty: Box::new(constants::VOID_PTR_TYPE.clone()),
        });

        let stmt = Stmt::Expr(ptr_cast_expr, None);
        return stmt;
    }

    fn generate_field_expr_ref_expr(&self, field: &Field) -> Expr {
        let field_ident = field
            .ident
            .clone()
            .expect("Call struct fields should have ident.");
        let field_access = self.expr_reference_factory.create(
            self.field_access_expr_factory
                .create(vec![constants::SELF_IDENT.clone(), field_ident.clone()]),
        );
        let field_reference = self.expr_reference_factory.create(field_access);
        return field_reference;
    }
}
