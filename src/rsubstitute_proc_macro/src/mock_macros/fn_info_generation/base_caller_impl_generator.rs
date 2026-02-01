use crate::constants;
use crate::mock_macros::fn_info_generation::models::*;
use crate::mock_macros::mock_generation::models::*;
use crate::mock_macros::models::FnDecl;
use crate::syntax::*;
use quote::format_ident;
use std::cell::LazyCell;
use std::sync::Arc;
use syn::*;

pub trait IBaseCallerImplGenerator {
    fn generate(
        &self,
        mock_type: &MockType,
        fn_decl: &FnDecl,
        call_struct: &CallStruct,
        base_impl_fn_block: Block,
    ) -> BaseCallerImpl;
}

pub(crate) struct BaseCallerImplGenerator {
    pub type_factory: Arc<dyn ITypeFactory>,
    pub path_factory: Arc<dyn IPathFactory>,
}

impl IBaseCallerImplGenerator for BaseCallerImplGenerator {
    fn generate(
        &self,
        mock_type: &MockType,
        fn_decl: &FnDecl,
        call_struct: &CallStruct,
        base_impl_fn_block: Block,
    ) -> BaseCallerImpl {
        let trait_path = Path {
            leading_colon: None,
            segments: [PathSegment {
                ident: constants::I_BASE_CALLER_TRAIT_IDENT.clone(),
                arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                    colon2_token: None,
                    lt_token: Default::default(),
                    args: [
                        GenericArgument::Type(
                            self.type_factory
                                .create_from_struct(&call_struct.item_struct),
                        ),
                        GenericArgument::Type(fn_decl.get_return_value_type()),
                    ]
                    .into_iter()
                    .collect(),
                    gt_token: Default::default(),
                }),
            }]
            .into_iter()
            .collect(),
        };
        let call_base_fn =
            self.generate_call_base_fn(mock_type, fn_decl, call_struct, base_impl_fn_block);
        let item_impl = ItemImpl {
            attrs: Vec::new(),
            defaultness: None,
            unsafety: None,
            impl_token: Default::default(),
            generics: mock_type.generics.impl_generics.clone(),
            trait_: Some((None, trait_path, Default::default())),
            self_ty: Box::new(mock_type.ty.clone()),
            brace_token: Default::default(),
            items: vec![call_base_fn],
        };

        let base_caller_impl = BaseCallerImpl { item_impl };
        return base_caller_impl;
    }
}

impl BaseCallerImplGenerator {
    const CALL_ARG_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("call"));

    fn generate_call_base_fn(
        &self,
        mock_type: &MockType,
        fn_decl: &FnDecl,
        call_struct: &CallStruct,
        base_impl_fn_block: Block,
    ) -> ImplItem {
        let sig = Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: Default::default(),
            ident: constants::I_BASE_CALLER_CALL_BASE_FN_IDENT.clone(),
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
                    ty: Box::new(
                        self.type_factory
                            .create_from_struct(&call_struct.item_struct),
                    ),
                }),
            ]
            .into_iter()
            .collect(),
            variadic: None,
            output: fn_decl.return_value.clone(),
        };
        let block = self.generate_call_base_fn_block(mock_type, call_struct, base_impl_fn_block);
        let impl_item_fn = ImplItemFn {
            attrs: Vec::new(),
            vis: Visibility::Inherited,
            defaultness: None,
            sig,
            block,
        };
        let impl_item = ImplItem::Fn(impl_item_fn);
        return impl_item;
    }

    fn generate_call_base_fn_block(
        &self,
        mock_type: &MockType,
        call_struct: &CallStruct,
        base_impl_fn_block: Block,
    ) -> Block {
        let fields = call_struct
            .item_struct
            .fields
            .iter()
            // TODO - replace it with just get_phantom_types_count? I think I know that there's always one extra phantom for lifetime and can add it in get_phantom_types_count()?
            .skip(1 + mock_type.generics.get_phantom_types_count())
            .map(|field| FieldPat {
                attrs: Vec::new(),
                member: Member::Named(field.get_required_ident()),
                colon_token: None,
                pat: Box::new(Pat::Wild(PatWild {
                    attrs: Vec::new(),
                    underscore_token: Default::default(),
                })),
            })
            .collect();
        let deconstruct_call_stmt = Stmt::Local(Local {
            attrs: Vec::new(),
            let_token: Default::default(),
            pat: Pat::Struct(PatStruct {
                attrs: Vec::new(),
                qself: None,
                path: self
                    .path_factory
                    .create(call_struct.item_struct.ident.clone()),
                brace_token: Default::default(),
                fields,
                rest: Some(PatRest {
                    attrs: Vec::new(),
                    dot2_token: Default::default(),
                }),
            }),
            init: Some(LocalInit {
                eq_token: Default::default(),
                expr: Box::new(self.path_factory.create_expr(Self::CALL_ARG_IDENT.clone())),
                diverge: None,
            }),
            semi_token: Default::default(),
        });
        let stmts = std::iter::once(deconstruct_call_stmt)
            .chain(base_impl_fn_block.stmts)
            .collect();
        let block = Block {
            brace_token: Default::default(),
            stmts,
        };
        return block;
    }
}
