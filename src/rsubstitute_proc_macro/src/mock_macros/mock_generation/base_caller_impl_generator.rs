// use crate::constants;
// use crate::mock_macros::fn_info_generation::models::*;
// use crate::mock_macros::mock_generation::models::*;
// use crate::mock_macros::models::FnDecl;
// use crate::syntax::*;
// use quote::format_ident;
// use std::cell::LazyCell;
// use std::sync::Arc;
// use syn::*;
// 
// pub trait IBaseCallerImplGenerator {
//     fn generate(
//         &self,
//         base_caller_struct: &BaseCallerStruct,
//         call_struct: &CallStruct,
//         fn_decl: &FnDecl,
//         base_fn: &BaseFn,
//         phantom_types_count: usize,
//     ) -> BaseCallerImpl;
// }
// 
// pub(crate) struct BaseCallerImplGenerator {
//     pub type_factory: Arc<dyn ITypeFactory>,
//     pub path_factory: Arc<dyn IPathFactory>,
//     pub field_access_expr_factory: Arc<dyn IFieldAccessExprFactory>,
// }
// 
// impl IBaseCallerImplGenerator for BaseCallerImplGenerator {
//     fn generate(
//         &self,
//         base_caller_struct: &BaseCallerStruct,
//         call_struct: &CallStruct,
//         fn_decl: &FnDecl,
//         base_fn: &BaseFn,
//         phantom_types_count: usize,
//     ) -> BaseCallerImpl {
//         let trait_path = Path {
//             leading_colon: None,
//             segments: [PathSegment {
//                 ident: constants::I_BASE_CALLER_TRAIT_IDENT.clone(),
//                 arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
//                     colon2_token: None,
//                     lt_token: Default::default(),
//                     args: [
//                         GenericArgument::Type(
//                             self.type_factory
//                                 .create_from_struct(&call_struct.item_struct),
//                         ),
//                         GenericArgument::Type(fn_decl.get_return_value_type()),
//                     ]
//                     .into_iter()
//                     .collect(),
//                     gt_token: Default::default(),
//                 }),
//             }]
//             .into_iter()
//             .collect(),
//         };
//         let self_ty = self
//             .type_factory
//             .create_from_struct(&base_caller_struct.item_struct);
//         let items = self.generate_impl_items(fn_decl, call_struct, base_fn, phantom_types_count);
//         let item_impl = ItemImpl {
//             attrs: Vec::new(),
//             defaultness: None,
//             unsafety: None,
//             impl_token: Default::default(),
//             generics: call_struct.item_struct.generics.clone(),
//             trait_: Some((None, trait_path, Default::default())),
//             self_ty: Box::new(self_ty),
//             brace_token: Default::default(),
//             items,
//         };
//         let base_caller_impl = BaseCallerImpl { item_impl };
//         return base_caller_impl;
//     }
// }
// 
// impl BaseCallerImplGenerator {
//     const CALL_ARG_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("call"));
// 
//     fn generate_impl_items(
//         &self,
//         fn_decl: &FnDecl,
//         call_struct: &CallStruct,
//         base_fn: &BaseFn,
//         phantom_types_count: usize,
//     ) -> Vec<ImplItem> {
//         let impl_item_fn = ImplItemFn {
//             attrs: Vec::new(),
//             vis: Visibility::Inherited,
//             defaultness: None,
//             sig: Signature {
//                 constness: None,
//                 asyncness: None,
//                 unsafety: None,
//                 abi: None,
//                 fn_token: Default::default(),
//                 ident: constants::I_BASE_CALLER_CALL_BASE_FN_IDENT.clone(),
//                 generics: Generics::default(),
//                 paren_token: Default::default(),
//                 inputs: [
//                     constants::REF_SELF_ARG.clone(),
//                     FnArg::Typed(PatType {
//                         attrs: Vec::new(),
//                         pat: Box::new(Pat::Ident(PatIdent {
//                             attrs: Vec::new(),
//                             by_ref: None,
//                             mutability: None,
//                             ident: Self::CALL_ARG_IDENT.clone(),
//                             subpat: None,
//                         })),
//                         colon_token: Default::default(),
//                         ty: Box::new(
//                             self.type_factory
//                                 .create_from_struct(&call_struct.item_struct),
//                         ),
//                     }),
//                 ]
//                 .into_iter()
//                 .collect(),
//                 variadic: None,
//                 output: fn_decl.return_value.clone(),
//             },
//             block: Block {
//                 brace_token: Default::default(),
//                 stmts: vec![Stmt::Expr(
//                     Expr::Return(ExprReturn {
//                         attrs: Vec::new(),
//                         return_token: Default::default(),
//                         expr: Some(Box::new(Expr::Call(ExprCall {
//                             attrs: Vec::new(),
//                             func: Box::new(
//                                 self.path_factory
//                                     .create_expr(base_fn.item_fn.sig.ident.clone()),
//                             ),
//                             paren_token: Default::default(),
//                             args: call_struct
//                                 .item_struct
//                                 .fields
//                                 .iter()
//                                 .skip(1 + phantom_types_count)
//                                 .map(|field| {
//                                     self.field_access_expr_factory.create(vec![
//                                         Self::CALL_ARG_IDENT.clone(),
//                                         field.get_required_ident(),
//                                     ])
//                                 })
//                                 .collect(),
//                         }))),
//                     }),
//                     None,
//                 )],
//             },
//         };
//         let impl_items = vec![ImplItem::Fn(impl_item_fn)];
//         return impl_items;
//     }
// }
