use crate::mock_macros::constants;
use crate::mock_macros::fn_info_generation::models::{ArgsCheckerImplInfo, ArgsCheckerInfo, CallInfo};
use crate::syntax::{IFieldAccessExprFactory, ITypeFactory};
use proc_macro2::{Ident, Span};
use quote::format_ident;
use std::cell::LazyCell;
use std::rc::Rc;
use syn::{
    AngleBracketedGenericArguments, BinOp, Block, Expr, ExprBinary, ExprLit, ExprMethodCall, Field,
    FnArg, GenericArgument, Generics, ImplItem, ImplItemFn, ItemImpl, Lit, LitBool, Pat, PatIdent,
    PatType, Path, PathArguments, PathSegment, ReturnType, Signature, Stmt, Type, Visibility,
};

pub trait IArgsCheckerImplGenerator {
    fn generate(
        &self,
        call_info: &CallInfo,
        args_checker_info: &ArgsCheckerInfo,
    ) -> ArgsCheckerImplInfo;
}

pub struct ArgsCheckerImplGenerator {
    pub(crate) type_factory: Rc<dyn ITypeFactory>,
    pub(crate) field_access_expr_factory: Rc<dyn IFieldAccessExprFactory>,
}

impl IArgsCheckerImplGenerator for ArgsCheckerImplGenerator {
    fn generate(
        &self,
        call_info: &CallInfo,
        args_checker_info: &ArgsCheckerInfo,
    ) -> ArgsCheckerImplInfo {
        let trait_ident = constants::I_ARGS_CHECKER_TRAIT_IDENT.clone();
        let trait_path = Path {
            leading_colon: None,
            segments: [PathSegment {
                ident: trait_ident,
                arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                    colon2_token: None,
                    lt_token: Default::default(),
                    args: [GenericArgument::Type(
                        self.type_factory
                            .create(call_info.item_struct.ident.clone()),
                    )]
                    .into_iter()
                    .collect(),
                    gt_token: Default::default(),
                }),
            }]
            .into_iter()
            .collect(),
        };
        let call_ty = Box::new(
            self.type_factory
                .create(call_info.item_struct.ident.clone()),
        );
        let self_ty = Box::new(
            self.type_factory
                .create(args_checker_info.item_struct.ident.clone()),
        );
        let items = self.generate_matches_fn(call_info, call_ty);
        let item_impl = ItemImpl {
            attrs: Vec::new(),
            defaultness: None,
            unsafety: None,
            impl_token: Default::default(),
            generics: Default::default(),
            trait_: Some((None, trait_path, Default::default())),
            self_ty,
            brace_token: Default::default(),
            items: vec![items],
        };
        let args_checker_impl_info = ArgsCheckerImplInfo { item_impl };
        return args_checker_impl_info;
    }
}

impl ArgsCheckerImplGenerator {
    const MATCHES_FN_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("matches"));

    // TODO - test that equals to Arg::matches
    const ARG_MATCHES_FN_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("matches"));
    // TODO - test that equals to Arg::matches_ref
    const ARG_MATCHES_REF_FN_IDENT: LazyCell<Ident> =
        LazyCell::new(|| format_ident!("matches_ref"));
    // TODO - test that equals to Arg::matches_rc
    const ARG_MATCHES_RC_FN_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("matches_rc"));
    // TODO - test that equals to Arg::matches_arc
    const ARG_MATCHES_ARC_FN_IDENT: LazyCell<Ident> =
        LazyCell::new(|| format_ident!("matches_arc"));

    const CALL_ARG_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("call"));

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
                inputs: [
                    constants::REF_SELF_ARG.clone(),
                    FnArg::Typed(PatType {
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
                    }),
                ]
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
        let field_ident = field
            .ident
            .clone()
            .expect("Call struct fields should have ident.");
        let receiver = self
            .field_access_expr_factory
            .create(&[constants::SELF_IDENT.clone(), field_ident.clone()]);
        let arg = self
            .field_access_expr_factory
            .create(&[Self::CALL_ARG_IDENT.clone(), field_ident]);
        let method = self.get_matches_fn_ident(&field.ty);
        let expr = Expr::MethodCall(ExprMethodCall {
            attrs: Vec::new(),
            receiver: Box::new(receiver),
            dot_token: Default::default(),
            method,
            turbofish: None,
            paren_token: Default::default(),
            args: [arg].into_iter().collect(),
        });
        return expr;
    }

    fn get_matches_fn_ident(&self, ty: &Type) -> Ident {
        if let Type::Reference(_) = ty {
            return Self::ARG_MATCHES_REF_FN_IDENT.clone();
        }
        if let Type::Path(type_path) = ty {
            if let Some(ident) = type_path.path.segments.last().map(|x| &x.ident) {
                println!("arg ident: {}", ident);
                if ident == "Rc" {
                    return Self::ARG_MATCHES_RC_FN_IDENT.clone();
                }
                if ident == "Arc" {
                    return Self::ARG_MATCHES_ARC_FN_IDENT.clone();
                }
            }
        }
        return Self::ARG_MATCHES_FN_IDENT.clone();
    }
}
