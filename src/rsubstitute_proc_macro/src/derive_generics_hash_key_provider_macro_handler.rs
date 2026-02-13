use crate::constants;
use crate::syntax::*;
use proc_macro::TokenStream;
use quote::{ToTokens, format_ident};
use std::cell::LazyCell;
use std::sync::Arc;
use syn::punctuated::Punctuated;
use syn::*;

pub trait IDeriveGenericsHashKeyProviderMacroHandler {
    fn handle(&self, item: proc_macro::TokenStream) -> proc_macro::TokenStream;
}

pub(crate) struct DeriveGenericsHashKeyProviderMacroHandler {
    pub type_factory: Arc<dyn ITypeFactory>,
    pub path_factory: Arc<dyn IPathFactory>,
    pub expr_method_call_factory: Arc<dyn IExprMethodCallFactory>,
}

impl IDeriveGenericsHashKeyProviderMacroHandler for DeriveGenericsHashKeyProviderMacroHandler {
    fn handle(&self, item: TokenStream) -> TokenStream {
        let item_struct = parse_macro_input!(item as ItemStruct);

        let impl_items = self.generate_impl_items(&item_struct);
        let item_impl = ItemImpl {
            attrs: Vec::new(),
            defaultness: None,
            unsafety: None,
            impl_token: Default::default(),
            generics: item_struct.generics.clone(),
            trait_: Some((
                None,
                constants::I_GENERICS_HASH_KEY_PROVIDER_TRAIT_PATH.clone(),
                Default::default(),
            )),
            self_ty: Box::new(self.type_factory.create_from_struct(&item_struct)),
            brace_token: Default::default(),
            items: vec![
                impl_items.hash_generics_type_ids_item,
                impl_items.hash_const_values_item,
            ],
        };

        return item_impl.into_token_stream().into();
    }
}

impl DeriveGenericsHashKeyProviderMacroHandler {
    const GENERICS_HASHER_IDENT: LazyCell<Ident> =
        LazyCell::new(|| format_ident!("GenericsHasher"));
    const HASHER_ARG_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("hasher"));
    const TID_FN_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("tid"));
    const CONST_HASH_FN_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("const_hash"));

    const HASH_GENERICS_TYPE_IDS_FN_IDENT: LazyCell<Ident> =
        LazyCell::new(|| format_ident!("hash_generics_type_ids"));
    const HASH_CONST_VALUES_FN_IDENT: LazyCell<Ident> =
        LazyCell::new(|| format_ident!("hash_const_values"));

    fn generate_impl_items(&self, item_struct: &ItemStruct) -> ImplItems {
        let mut type_params = Vec::new();
        let mut const_params = Vec::new();
        for generic_param in item_struct.generics.params.iter() {
            match generic_param {
                GenericParam::Type(type_param) => type_params.push(type_param),
                GenericParam::Const(const_param) => const_params.push(const_param),
                _ => (),
            }
        }
        let hash_generics_type_ids_item = self.generate_hash_generics_type_ids_item(type_params);
        let hash_const_values_item = self.generate_hash_const_values_item(const_params);

        let impl_items = ImplItems {
            hash_generics_type_ids_item,
            hash_const_values_item,
        };
        return impl_items;
    }

    fn generate_hash_generics_type_ids_item(&self, type_params: Vec<&TypeParam>) -> ImplItem {
        let sig = self.generate_hash_fn_sig(Self::HASH_GENERICS_TYPE_IDS_FN_IDENT.clone());
        let stmts = if type_params.len() > 0 {
            let tid_exprs = type_params.iter().map(|x| self.generate_tid_expr(x));
            let tid_array_expr = ExprArray {
                attrs: Vec::new(),
                bracket_token: Default::default(),
                elems: tid_exprs.collect(),
            };
            let hash_method_call = self.expr_method_call_factory.create_with_base_receiver(
                Expr::Array(tid_array_expr),
                Vec::new(),
                constants::HASH_FN_IDENT.clone(),
                vec![Self::HASHER_ARG_IDENT.clone()],
            );
            let stmt = Stmt::Expr(Expr::MethodCall(hash_method_call), None);
            vec![stmt]
        } else {
            Vec::new()
        };
        let impl_item_fn = ImplItemFn {
            attrs: Vec::new(),
            vis: Visibility::Inherited,
            defaultness: None,
            sig,
            block: Block {
                brace_token: Default::default(),
                stmts,
            },
        };
        return ImplItem::Fn(impl_item_fn);
    }

    fn generate_hash_const_values_item(&self, const_params: Vec<&ConstParam>) -> ImplItem {
        let sig = self.generate_hash_fn_sig(Self::HASH_CONST_VALUES_FN_IDENT.clone());
        let stmts = const_params
            .iter()
            .map(|x| self.generate_const_hash_stmt(x))
            .collect();
        let impl_item_fn = ImplItemFn {
            attrs: Vec::new(),
            vis: Visibility::Inherited,
            defaultness: None,
            sig,
            block: Block {
                brace_token: Default::default(),
                stmts,
            },
        };
        return ImplItem::Fn(impl_item_fn);
    }

    fn generate_hash_fn_sig(&self, fn_ident: Ident) -> Signature {
        let inputs = [
            constants::REF_SELF_ARG.clone(),
            FnArg::Typed(PatType {
                attrs: Vec::new(),
                pat: Box::new(Pat::Ident(PatIdent {
                    attrs: Vec::new(),
                    by_ref: None,
                    mutability: None,
                    ident: Self::HASHER_ARG_IDENT.clone(),
                    subpat: None,
                })),
                colon_token: Default::default(),
                ty: Box::new(
                    self.type_factory.mut_reference(
                        self.type_factory
                            .create(Self::GENERICS_HASHER_IDENT.clone()),
                        None,
                    ),
                ),
            }),
        ]
        .into_iter()
        .collect();
        let signature = Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: Default::default(),
            ident: fn_ident,
            generics: Generics::default(),
            paren_token: Default::default(),
            inputs,
            variadic: None,
            output: ReturnType::Default,
        };
        return signature;
    }

    fn generate_tid_expr(&self, type_param: &TypeParam) -> Expr {
        let expr_call = ExprCall {
            attrs: Vec::new(),
            func: Box::new(Expr::Path(ExprPath {
                attrs: Vec::new(),
                qself: None,
                path: Path {
                    leading_colon: None,
                    segments: [PathSegment {
                        ident: Self::TID_FN_IDENT.clone(),
                        arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                            colon2_token: Some(Default::default()),
                            lt_token: Default::default(),
                            args: [GenericArgument::Type(
                                self.type_factory.create(type_param.ident.clone()),
                            )]
                            .into_iter()
                            .collect(),
                            gt_token: Default::default(),
                        }),
                    }]
                    .into_iter()
                    .collect(),
                },
            })),
            paren_token: Default::default(),
            args: Punctuated::new(),
        };
        let expr = Expr::Call(expr_call);
        return expr;
    }

    fn generate_const_hash_stmt(&self, const_param: &ConstParam) -> Stmt {
        let expr_call = ExprCall {
            attrs: Vec::new(),
            func: Box::new(
                self.path_factory
                    .create_expr(Self::CONST_HASH_FN_IDENT.clone()),
            ),
            paren_token: Default::default(),
            args: [
                Expr::Reference(ExprReference {
                    attrs: Vec::new(),
                    and_token: Default::default(),
                    mutability: None,
                    expr: Box::new(self.path_factory.create_expr(const_param.ident.clone())),
                }),
                self.path_factory
                    .create_expr(Self::HASHER_ARG_IDENT.clone()),
            ]
            .into_iter()
            .collect(),
        };
        let stmt = Stmt::Expr(Expr::Call(expr_call), Some(Default::default()));
        return stmt;
    }
}

struct ImplItems {
    pub hash_generics_type_ids_item: ImplItem,
    pub hash_const_values_item: ImplItem,
}
