use crate::macros::constants;
use crate::macros::fn_info_generation::models::FnInfo;
use crate::macros::mock_generation::models::{InternalMockImplInfo, MockStructInfo};
use crate::macros::models::TargetDecl;
use crate::syntax::ITypeFactory;
use proc_macro2::Ident;
use quote::format_ident;
use std::cell::LazyCell;
use std::rc::Rc;
use syn::punctuated::Punctuated;
use syn::{
    Block, Expr, ExprStruct, Generics, ImplItem, ImplItemFn, ItemImpl, ReturnType, Signature, Stmt,
    Visibility,
};

pub trait IInternalMockImplGenerator {
    fn generate(
        &self,
        target_decl: &TargetDecl,
        mock_struct_info: &MockStructInfo,
        fn_infos: &[FnInfo],
    ) -> InternalMockImplInfo;
}

pub struct InternalMockImplGenerator {
    pub(crate) type_factory: Rc<dyn ITypeFactory>,
}

impl IInternalMockImplGenerator for InternalMockImplGenerator {
    fn generate(
        &self,
        target_decl: &TargetDecl,
        mock_struct_info: &MockStructInfo,
        fn_infos: &[FnInfo],
    ) -> InternalMockImplInfo {
        let self_ty = self
            .type_factory
            .create(mock_struct_info.item_struct.ident.clone());
        let item_impl = ItemImpl {
            attrs: Vec::new(),
            defaultness: None,
            unsafety: None,
            impl_token: Default::default(),
            generics: Default::default(),
            trait_: None,
            self_ty: Box::new(self_ty),
            brace_token: Default::default(),
            items,
        };

        let internal_mock_impl_info = InternalMockImplInfo { impl_item };
        return internal_mock_impl_info;
    }
}

impl InternalMockImplGenerator {
    const CONSTRUCTOR_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("new"));

    fn generate_constructor(
        &self,
        target_decl: &TargetDecl,
        mock_struct_info: &MockStructInfo,
    ) -> ImplItem {
        let block = self.generate_constructor_block(target_decl, mock_struct_info);
        let item_impl = ImplItem::Fn(ImplItemFn {
            attrs: Vec::new(),
            vis: Visibility::Public(Default::default()),
            defaultness: None,
            sig: Signature {
                constness: None,
                asyncness: None,
                unsafety: None,
                abi: None,
                fn_token: Default::default(),
                ident: Self::CONSTRUCTOR_IDENT.clone(),
                generics: Generics::default(),
                paren_token: Default::default(),
                inputs: Punctuated::new(),
                variadic: None,
                output: ReturnType::Type(
                    Default::default(),
                    Box::new(constants::SELF_TYPE.clone()),
                ),
            },
            block,
        });
        return item_impl;
    }

    fn generate_constructor_block(
        &self,
        target_decl: &TargetDecl,
        mock_struct_info: &MockStructInfo,
    ) -> Block {
        let block = Block {
            brace_token: Default::default(),
            stmts: vec![Stmt::Expr(
                Expr::Struct(ExprStruct {
                    attrs: Vec::new(),
                    qself: None,
                    path: constants::SELF_TYPE_PATH.clone(),
                    brace_token: Default::default(),
                    fields,
                    dot2_token: None,
                    rest: None,
                }),
                None,
            )],
        };
        todo!();
    }
}
