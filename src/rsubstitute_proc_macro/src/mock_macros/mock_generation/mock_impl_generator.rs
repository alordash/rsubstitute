use crate::mock_macros::constants;
use crate::mock_macros::fn_info_generation::models::FnInfo;
use crate::mock_macros::mock_generation::models::{MockImplInfo, MockStructInfo};
use crate::mock_macros::models::TargetDecl;
use crate::syntax::{IExprMethodCallFactory, IFieldValueFactory, IPathFactory, ITypeFactory};
use proc_macro2::Ident;
use quote::format_ident;
use std::cell::LazyCell;
use std::rc::Rc;
use syn::{
    Block, Expr, ExprReturn, ExprStruct, ImplItem, ImplItemFn, ItemImpl, Local, LocalInit, Pat,
    PatPath, Signature, Stmt, Visibility,
};

pub trait IMockImplGenerator {
    fn generate(
        &self,
        target_decl: &TargetDecl,
        mock_struct_info: &MockStructInfo,
        fn_infos: &[FnInfo],
    ) -> MockImplInfo;
}

pub struct MockImplGenerator {
    pub(crate) path_factory: Rc<dyn IPathFactory>,
    pub(crate) type_factory: Rc<dyn ITypeFactory>,
    pub(crate) field_value_factory: Rc<dyn IFieldValueFactory>,
    pub(crate) expr_method_call_factory: Rc<dyn IExprMethodCallFactory>,
}

impl IMockImplGenerator for MockImplGenerator {
    fn generate(
        &self,
        target_decl: &TargetDecl,
        mock_struct_info: &MockStructInfo,
        fn_infos: &[FnInfo],
    ) -> MockImplInfo {
        let trait_ = self.path_factory.create(target_decl.ident.clone());
        let self_ty = self
            .type_factory
            .create(mock_struct_info.item_struct.ident.clone());
        let items = fn_infos
            .iter()
            .map(|x| self.generate_impl_item_fn(x))
            .map(ImplItem::Fn)
            .collect();
        let item_impl = ItemImpl {
            attrs: Vec::new(),
            defaultness: None,
            unsafety: None,
            impl_token: Default::default(),
            generics: Default::default(),
            trait_: Some((None, trait_, Default::default())),
            self_ty: Box::new(self_ty),
            brace_token: Default::default(),
            items,
        };

        let mock_impl_info = MockImplInfo { item_impl };
        return mock_impl_info;
    }
}

impl MockImplGenerator {
    const CALL_VARIABLE_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("call"));
    const HANDLE_METHOD_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("handle")); // TODO - add test that it equals to FnData::handle
    const HANDLE_RETURNING_METHOD_IDENT: LazyCell<Ident> =
        LazyCell::new(|| format_ident!("handle_returning")); // TODO - add test that it equals to FnData::handle_returning

    fn generate_impl_item_fn(&self, fn_info: &FnInfo) -> ImplItemFn {
        let sig = Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: Default::default(),
            ident: fn_info.parent.ident.clone(),
            generics: Default::default(),
            paren_token: Default::default(),
            inputs: fn_info.parent.arguments.iter().cloned().collect(),
            variadic: None,
            output: fn_info.parent.return_value.clone(),
        };
        let block = self.generate_impl_item_fn_block(fn_info);
        let impl_item_fn = ImplItemFn {
            attrs: Vec::new(),
            vis: Visibility::Inherited,
            defaultness: None,
            sig,
            block,
        };
        return impl_item_fn;
    }

    fn generate_impl_item_fn_block(&self, fn_info: &FnInfo) -> Block {
        let call_stmt = self.generate_call_stmt(fn_info);
        let last_stmt = self.generate_last_stmt(fn_info);
        let stmts = vec![call_stmt, last_stmt];
        let block = Block {
            brace_token: Default::default(),
            stmts,
        };
        return block;
    }

    fn generate_call_stmt(&self, fn_info: &FnInfo) -> Stmt {
        let call_stmt = Stmt::Local(Local {
            attrs: Vec::new(),
            let_token: Default::default(),
            pat: Pat::Path(PatPath {
                attrs: Vec::new(),
                qself: None,
                path: self.path_factory.create(Self::CALL_VARIABLE_IDENT.clone()),
            }),
            init: Some(LocalInit {
                eq_token: Default::default(),
                expr: Box::new(Expr::Struct(ExprStruct {
                    attrs: Vec::new(),
                    qself: None,
                    path: self
                        .path_factory
                        .create(fn_info.call_info.item_struct.ident.clone()),
                    brace_token: Default::default(),
                    fields: fn_info
                        .call_info
                        .item_struct
                        .fields
                        .iter()
                        .map(|field| self.field_value_factory.create(field))
                        .collect(),
                    dot2_token: None,
                    rest: None,
                })),
                diverge: None,
            }),
            semi_token: Default::default(),
        });
        return call_stmt;
    }

    fn generate_last_stmt(&self, fn_info: &FnInfo) -> Stmt {
        let handle_expr = self.generate_handle_expr(fn_info);
        let last_expr = if fn_info.parent.has_return_value() {
            Expr::Return(ExprReturn {
                attrs: Vec::new(),
                return_token: Default::default(),
                expr: Some(Box::new(handle_expr)),
            })
        } else {
            handle_expr
        };
        let last_stmt = Stmt::Expr(last_expr, Some(Default::default()));
        return last_stmt;
    }

    fn generate_handle_expr(&self, fn_info: &FnInfo) -> Expr {
        let idents = [
            constants::SELF_IDENT.clone(),
            fn_info.data_field_ident.clone(),
        ];
        let method = if fn_info.parent.has_return_value() {
            Self::HANDLE_RETURNING_METHOD_IDENT.clone()
        } else {
            Self::HANDLE_METHOD_IDENT.clone()
        };
        let expr_method_call = self.expr_method_call_factory.create(
            &idents,
            method,
            &[Self::CALL_VARIABLE_IDENT.clone()],
        );
        let expr = Expr::MethodCall(expr_method_call);
        return expr;
    }
}
