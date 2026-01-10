use crate::constants;
use crate::lifetime_ref::LifetimeRef;
use crate::mock_macros::fn_info_generation::models::FnInfo;
use crate::mock_macros::mock_generation::models::{MockTraitImpl, MockStruct};
use crate::syntax::*;
use proc_macro2::Ident;
use quote::format_ident;
use std::cell::LazyCell;
use std::sync::Arc;
use syn::punctuated::Punctuated;
use syn::*;

pub trait IMockTraitImplGenerator {
    fn generate(
        &self,
        target_ident: Ident,
        mock_struct: &MockStruct,
        fn_infos: &[FnInfo],
    ) -> MockTraitImpl;
}

pub(crate) struct MockTraitImplGenerator {
    pub path_factory: Arc<dyn IPathFactory>,
    pub type_factory: Arc<dyn ITypeFactory>,
    pub expr_method_call_factory: Arc<dyn IExprMethodCallFactory>,
    pub std_mem_transmute_expr_factory: Arc<dyn IStdMemTransmuteExprFactory>,
    pub reference_normalizer: Arc<dyn IReferenceNormalizer>,
    pub reference_type_crawler: Arc<dyn IReferenceTypeCrawler>,
}

impl IMockTraitImplGenerator for MockTraitImplGenerator {
    fn generate(
        &self,
        target_ident: Ident,
        mock_struct: &MockStruct,
        fn_infos: &[FnInfo],
    ) -> MockTraitImpl {
        let trait_ = self.path_factory.create(target_ident);
        let self_ty = self
            .type_factory
            .create(mock_struct.item_struct.ident.clone());
        let items = fn_infos
            .iter()
            .map(|x| self.generate_impl_item_fn(x))
            .map(ImplItem::Fn)
            .collect();

        let mut item_impl = ItemImpl {
            attrs: Vec::new(),
            defaultness: None,
            unsafety: None,
            impl_token: Default::default(),
            generics: constants::DEFAULT_ARG_FIELD_LIFETIME_GENERIC.clone(),
            trait_: Some((None, trait_, Default::default())),
            self_ty: Box::new(self_ty),
            brace_token: Default::default(),
            items,
        };
        self.reference_normalizer.normalize_in_impl(
            constants::DEFAULT_ARG_FIELD_LIFETIME.clone(),
            &mut item_impl,
        );
        let mock_impl = MockTraitImpl { item_impl };
        return mock_impl;
    }
}

impl MockTraitImplGenerator {
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
            generics: Generics {
                lt_token: Some(Default::default()),
                params: [GenericParam::Lifetime(LifetimeParam {
                    attrs: Vec::new(),
                    lifetime: constants::ANONYMOUS_LIFETIME.clone(),
                    colon_token: None,
                    bounds: Punctuated::new(),
                })]
                .into_iter()
                .collect(),
                gt_token: Some(Default::default()),
                where_clause: None,
            },
            paren_token: Default::default(),
            inputs: fn_info
                .parent
                .arguments
                .clone()
                .into_iter()
                .map(|mut fn_arg| {
                    self.convert_input_reference(&mut fn_arg);
                    return fn_arg;
                })
                .collect(),
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

    fn convert_input_reference(&self, fn_arg: &mut FnArg) {
        let ty = match fn_arg {
            FnArg::Receiver(receiver) => {
                if let Some((_, lifetime)) = &mut receiver.reference {
                    self.anonymize_input_reference_lifetime(LifetimeRef::Optional(lifetime));
                }
                receiver.ty.as_mut()
            }
            FnArg::Typed(pat_type) => pat_type.ty.as_mut(),
        };
        let lifetime_refs = self.reference_type_crawler.get_all_type_references(ty);
        for lifetime_ref in lifetime_refs {
            self.anonymize_input_reference_lifetime(lifetime_ref);
        }
    }

    fn anonymize_input_reference_lifetime(&self, lifetime_ref: LifetimeRef) {
        if let LifetimeRef::Optional(optional_lifetime) = lifetime_ref
            && optional_lifetime.is_none()
        {
            *optional_lifetime = Some(constants::ANONYMOUS_LIFETIME.clone());
        }
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
        let fn_fields: Vec<_> = fn_info
            .call_struct
            .item_struct
            .fields
            .iter()
            .skip(1)
            .map(|field| {
                let field_ident = field.ident.clone().expect("TODO field ident");
                FieldValue {
                    attrs: Vec::new(),
                    member: Member::Named(field_ident.clone()),
                    colon_token: Some(Default::default()),
                    expr: self.std_mem_transmute_expr_factory.create(field_ident),
                }
            })
            .collect();
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
                expr: Box::new(Expr::Unsafe(ExprUnsafe {
                    attrs: Vec::new(),
                    unsafe_token: Default::default(),
                    block: Block {
                        brace_token: Default::default(),
                        stmts: vec![Stmt::Expr(
                            Expr::Struct(ExprStruct {
                                attrs: Vec::new(),
                                qself: None,
                                path: self
                                    .path_factory
                                    .create(fn_info.call_struct.item_struct.ident.clone()),
                                brace_token: Default::default(),
                                fields: std::iter::once(
                                    constants::DEFAULT_ARG_FIELD_LIFETIME_FIELD_VALUE.clone(),
                                )
                                .chain(fn_fields)
                                .collect(),
                                dot2_token: None,
                                rest: None,
                            }),
                            None,
                        )],
                    },
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
        let idents = vec![
            constants::SELF_IDENT.clone(),
            constants::DATA_IDENT.clone(),
            fn_info.data_field_ident.clone(),
        ];
        let method = if fn_info.parent.has_return_value() {
            Self::HANDLE_RETURNING_METHOD_IDENT.clone()
        } else {
            Self::HANDLE_METHOD_IDENT.clone()
        };
        let expr_method_call = self.expr_method_call_factory.create(
            idents,
            method,
            vec![Self::CALL_VARIABLE_IDENT.clone()],
        );
        let expr = Expr::MethodCall(expr_method_call);
        return expr;
    }
}
