use crate::macros::constants;
use crate::macros::fn_info_generation::models::{ArgsMatcherImplInfo, ArgsMatcherInfo, CallInfo};
use crate::macros::models::FnDecl;
use crate::syntax::ITypeFactory;
use proc_macro2::{Ident, Span};
use quote::format_ident;
use std::cell::LazyCell;
use std::rc::Rc;
use syn::punctuated::Punctuated;
use syn::{
    BinOp, Block, Expr, ExprBinary, ExprField, ExprLit, ExprMethodCall, ExprPath, Field, FnArg,
    GenericParam, Generics, ImplItem, ImplItemFn, ItemImpl, Lit, LitBool, Member, Pat, PatIdent,
    PatType, Path, PathArguments, PathSegment, ReturnType, Signature, Stmt, Type, TypeParam,
    TypePath, Visibility,
};

pub trait IArgsMatcherImplGenerator {
    fn generate<'a>(
        &self,
        fn_decl: &'a FnDecl,
        call_info: &CallInfo,
        args_matcher_info: &ArgsMatcherInfo,
    ) -> ArgsMatcherImplInfo<'a>;
}

pub struct ArgsMatcherImplGenerator {
    pub(crate) type_factory: Rc<dyn ITypeFactory>,
}

impl IArgsMatcherImplGenerator for ArgsMatcherImplGenerator {
    fn generate<'a>(
        &self,
        fn_decl: &'a FnDecl,
        call_info: &CallInfo,
        args_matcher_info: &ArgsMatcherInfo,
    ) -> ArgsMatcherImplInfo<'a> {
        let call_info_ident = &call_info.item_struct.ident;
        let generics = Generics {
            lt_token: Default::default(),
            params: [GenericParam::Type(TypeParam {
                attrs: Vec::new(),
                ident: call_info_ident.clone(),
                colon_token: None,
                bounds: Punctuated::new(),
                eq_token: None,
                default: None,
            })]
            .into_iter()
            .collect(),
            gt_token: None,
            where_clause: None,
        };
        let trait_ident = constants::I_ARGS_MATCHER_TRAIT_IDENT.clone();
        let trait_path = Path {
            leading_colon: None,
            segments: [PathSegment {
                ident: trait_ident,
                arguments: PathArguments::None,
            }]
            .into_iter()
            .collect(),
        };
        let call_ty = Box::new(
            self.type_factory
                .create(args_matcher_info.item_struct.ident.clone()),
        );
        let items = self.generate_matches_fn(call_info, call_ty.clone());
        let item_impl = ItemImpl {
            attrs: Vec::new(),
            defaultness: None,
            unsafety: None,
            impl_token: Default::default(),
            generics,
            trait_: Some((None, trait_path, Default::default())),
            self_ty: call_ty,
            brace_token: Default::default(),
            items: vec![items],
        };
        let args_matcher_impl_info = ArgsMatcherImplInfo {
            parent: fn_decl,
            item_impl,
        };
        return args_matcher_impl_info;
    }
}

impl ArgsMatcherImplGenerator {
    const MATCHES_FN_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("matches"));
    const CALL_ARG_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("matches"));

    fn generate_matches_fn(&self, call_info: &CallInfo, call_type: Box<Type>) -> ImplItem {
        let matches_stmt = self.generate_matches_statement(call_info);
        let block = Block {
            brace_token: Default::default(),
            stmts: vec![matches_stmt],
        };
        let impl_item = ImplItem::Fn(ImplItemFn {
            attrs: Vec::new(),
            vis: Visibility::Inherited,
            defaultness: None,
            sig: Signature {
                constness: None,
                asyncness: None,
                unsafety: None,
                abi: None,
                fn_token: Default::default(),
                ident: Self::MATCHES_FN_IDENT.clone(),
                generics: Generics::default(),
                paren_token: Default::default(),
                inputs: [FnArg::Typed(PatType {
                    attrs: Vec::new(),
                    pat: Box::new(Pat::Ident(PatIdent {
                        attrs: Vec::new(),
                        by_ref: None,
                        mutability: None,
                        ident: Self::CALL_ARG_IDENT.clone(),
                        subpat: None,
                    })),
                    colon_token: Default::default(),
                    ty: call_type.clone(),
                })]
                .into_iter()
                .collect(),
                variadic: None,
                output: ReturnType::Type(
                    Default::default(),
                    Box::new(constants::BOOL_TYPE.clone()),
                ),
            },
            block,
        });
        return impl_item;
    }

    fn generate_matches_statement(&self, call_info: &CallInfo) -> Stmt {
        if call_info.item_struct.fields.len() == 0 {
            let true_stmt = Stmt::Expr(
                Expr::Lit(ExprLit {
                    attrs: Vec::new(),
                    lit: Lit::Bool(LitBool {
                        value: true,
                        span: Span::call_site(),
                    }),
                }),
                Default::default(),
            );
            return true_stmt;
        }

        let matches_exprs: Vec<_> = call_info
            .item_struct
            .fields
            .iter()
            .map(|field| self.generate_matches_exprs(field))
            .collect();
        let joined_exprs = matches_exprs
            .iter()
            .cloned()
            .reduce(|a, b| {
                Expr::Binary(ExprBinary {
                    attrs: Vec::new(),
                    left: Box::new(a),
                    op: BinOp::And(Default::default()),
                    right: Box::new(b),
                })
            })
            .expect("Should have at least one 'matches' statement.");
        let stmt = Stmt::Expr(joined_exprs, Default::default());
        return stmt;
    }

    fn generate_matches_exprs(&self, field: &Field) -> Expr {
        let expr = Expr::MethodCall(ExprMethodCall {
            attrs: Vec::new(),
            receiver: Box::new(Expr::Field(ExprField {
                attrs: Vec::new(),
                base: Box::new(Expr::Path(ExprPath {
                    attrs: Vec::new(),
                    qself: None,
                    path: Path {
                        leading_colon: None,
                        segments: [PathSegment {
                            ident: constants::SELF_IDENT.clone(),
                            arguments: PathArguments::None,
                        }]
                        .into_iter()
                        .collect(),
                    },
                })),
                dot_token: Default::default(),
                member: Member::Named(
                    field
                        .ident
                        .clone()
                        .expect("Call struct fields should have ident."),
                ),
            })),
            dot_token: Default::default(),
            method: Self::MATCHES_FN_IDENT.clone(),
            turbofish: None,
            paren_token: Default::default(),
            args: Punctuated::new(),
        });
        return expr;
    }
}
