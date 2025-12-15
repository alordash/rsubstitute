use crate::macros::fn_info_generation::models::FnInfo;
use crate::macros::mock_generation::models::{MockImplInfo, MockStructInfo};
use crate::macros::models::TargetDecl;
use crate::syntax::{IPathFactory, ITypeFactory};
use proc_macro2::Ident;
use quote::format_ident;
use std::cell::LazyCell;
use std::rc::Rc;
use syn::{
    Block, Expr, ExprMethodCall, ExprPath, ExprReturn, ExprStruct, FieldValue, ImplItem,
    ImplItemFn, ItemImpl, Local, LocalInit, Member, Pat, PatPath, Path, ReturnType, Signature,
    Stmt, Visibility,
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
    path_factory: Rc<dyn IPathFactory>,
    type_factory: Rc<dyn ITypeFactory>,
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
        todo!()
    }
}

impl MockImplGenerator {
    const CALL_VARIABLE_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("call"));

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
            output: fn_info
                .parent
                .return_value
                .clone()
                .map(|x| ReturnType::Type(Default::default(), x))
                .unwrap_or(ReturnType::Default),
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
        let return_stmt = self.generate_return_stmt(fn_info);
        let stmts = vec![call_stmt, return_stmt];
        let block = Block {
            brace_token: Default::default(),
            stmts,
        };
        todo!();
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
                        .map(|field| {
                            let field_ident = field
                                .ident
                                .clone()
                                .expect("Field in call struct should be named");
                            FieldValue {
                                attrs: Vec::new(),
                                member: Member::Named(field_ident.clone()),
                                colon_token: None,
                                expr: Expr::Path(ExprPath {
                                    attrs: Vec::new(),
                                    qself: None,
                                    path: self.path_factory.create(field_ident),
                                }),
                            }
                        })
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

    fn generate_return_stmt(&self, fn_info: &FnInfo) -> Stmt {
        let return_expr = Expr::Return(ExprReturn {
            attrs: Vec::new(),
            return_token: Default::default(),
            expr: Some(Box::new(Expr::MethodCall(ExprMethodCall {
                attrs: Vec::new(),
                receiver: todo!(),
            }))),
        });
        let return_stmt = Stmt::Expr(return_expr, Some(Default::default()));
        return return_stmt;
    }
}
