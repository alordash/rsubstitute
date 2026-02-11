use crate::constants;
use crate::mock_macros::fn_info_generation::models::*;
use crate::syntax::*;
use proc_macro2::{Ident, Span};
use quote::{ToTokens, format_ident};
use std::cell::LazyCell;
use std::sync::Arc;
use syn::punctuated::Punctuated;
use syn::token::Bracket;
use syn::*;

pub trait IArgsCheckerTraitImplGenerator {
    fn generate(
        &self,
        call_struct: &CallStruct,
        args_checker_struct: &ArgsCheckerStruct,
        phantom_types_count: usize,
    ) -> ArgsCheckerTraitImpl;
}

pub(crate) struct ArgsCheckerTraitImplGenerator {
    pub type_factory: Arc<dyn ITypeFactory>,
    pub field_access_expr_factory: Arc<dyn IFieldAccessExprFactory>,
}

impl IArgsCheckerTraitImplGenerator for ArgsCheckerTraitImplGenerator {
    fn generate(
        &self,
        call_struct: &CallStruct,
        args_checker_struct: &ArgsCheckerStruct,
        phantom_types_count: usize,
    ) -> ArgsCheckerTraitImpl {
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
                            .create_from_struct(&call_struct.item_struct),
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
                .create_from_struct(&call_struct.item_struct),
        );
        let self_ty = Box::new(
            self.type_factory
                .create_from_struct(&args_checker_struct.item_struct),
        );
        let items = self.generate_check_fn(call_struct, call_ty, phantom_types_count);
        let item_impl = ItemImpl {
            attrs: Vec::new(),
            defaultness: None,
            unsafety: None,
            impl_token: Default::default(),
            generics: args_checker_struct.item_struct.generics.clone(),
            trait_: Some((None, trait_path, Default::default())),
            self_ty,
            brace_token: Default::default(),
            items: vec![items],
        };
        let args_checker_impl = ArgsCheckerTraitImpl { item_impl };
        return args_checker_impl;
    }
}

impl ArgsCheckerTraitImplGenerator {
    const CHECK_FN_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("check"));

    const ARG_CHECK_FN_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("check"));
    const ARG_CHECK_REF_FN_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("check_ref"));
    const ARG_CHECK_MUT_REF_FN_IDENT: LazyCell<Ident> =
        LazyCell::new(|| format_ident!("check_mut"));
    const ARG_CHECK_RC_FN_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("check_rc"));
    const ARG_CHECK_ARC_FN_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("check_arc"));

    const CALL_ARG_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("call"));

    fn generate_check_fn(
        &self,
        call_struct: &CallStruct,
        call_type: Box<Type>,
        phantom_types_count: usize,
    ) -> ImplItem {
        let check_stmt = self.generate_check_stmt(call_struct, phantom_types_count);
        let block = Block {
            brace_token: Default::default(),
            stmts: vec![check_stmt],
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
                ident: Self::CHECK_FN_IDENT.clone(),
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
                    Box::new(constants::VEC_OF_ARG_CHECK_RESULT_TYPE.clone()),
                ),
            },
            block,
        });
        return impl_item;
    }

    fn generate_check_stmt(&self, call_struct: &CallStruct, phantom_types_count: usize) -> Stmt {
        let check_exprs: Punctuated<_, Token![,]> = call_struct
            .item_struct
            .fields
            .iter()
            .skip(1 + phantom_types_count)
            .map(|field| self.generate_check_exprs(field))
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

    fn generate_check_exprs(&self, field: &Field) -> Expr {
        let field_ident = field.get_required_ident();
        let receiver = self
            .field_access_expr_factory
            .create(vec![constants::SELF_IDENT.clone(), field_ident.clone()]);
        let field_name_arg = Expr::Lit(ExprLit {
            attrs: Vec::new(),
            lit: Lit::Str(LitStr::new(&field_ident.to_string(), Span::call_site())),
        });
        let field_access_arg = self
            .field_access_expr_factory
            .create(vec![Self::CALL_ARG_IDENT.clone(), field_ident]);
        let method = self.get_check_fn_ident(&field.ty);
        let expr = Expr::MethodCall(ExprMethodCall {
            attrs: Vec::new(),
            receiver: Box::new(receiver),
            dot_token: Default::default(),
            method,
            turbofish: None,
            paren_token: Default::default(),
            args: [field_name_arg, field_access_arg].into_iter().collect(),
        });
        return expr;
    }

    fn get_check_fn_ident(&self, ty: &Type) -> Ident {
        if let Type::Reference(_) = ty {
            return Self::ARG_CHECK_REF_FN_IDENT.clone();
        }
        if let Type::Ptr(ptr) = ty
            && ptr.mutability.is_some()
        {
            return Self::ARG_CHECK_MUT_REF_FN_IDENT.clone();
        }
        if let Type::Path(type_path) = ty {
            if let Some(ident) = type_path.path.segments.last().map(|x| &x.ident) {
                if ident == "Rc" {
                    return Self::ARG_CHECK_RC_FN_IDENT.clone();
                }
                if ident == "Arc" {
                    return Self::ARG_CHECK_ARC_FN_IDENT.clone();
                }
            }
        }
        return Self::ARG_CHECK_FN_IDENT.clone();
    }
}
