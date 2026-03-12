use crate::constants;
use crate::mock_macros::fn_info_generation::models::*;
use crate::mock_macros::mock_generation::models::*;
use crate::mock_macros::mock_generation::*;
use crate::mock_macros::models::FnDecl;
use crate::syntax::*;
use quote::format_ident;
use std::cell::LazyCell;
use std::sync::Arc;
use syn::*;

pub(crate) trait IBaseFnGenerator {
    fn generate(
        &self,
        mock_type: &MockType,
        fn_decl: &FnDecl,
        call_struct: &CallStruct,
        base_fn_block: Block,
    ) -> BaseFn;

    fn generate_struct_trait_fn(
        &self,
        mock_type: &MockType,
        fn_decl: &FnDecl,
        call_struct: &CallStruct,
        base_fn_block: Block,
        trait_ident: &Ident,
    ) -> BaseFn;

    fn generate_static(
        &self,
        mock_type: &MockType,
        fn_decl: &FnDecl,
        call_struct: &CallStruct,
        base_fn_block: Block,
    ) -> StaticBaseFn;
}

pub(crate) struct BaseFnGenerator {
    pub base_fn_ident_formatter: Arc<dyn IBaseFnIdentFormatter>,
}

impl IBaseFnGenerator for BaseFnGenerator {
    fn generate(
        &self,
        mock_type: &MockType,
        fn_decl: &FnDecl,
        call_struct: &CallStruct,
        base_fn_block: Block,
    ) -> BaseFn {
        let (sig, block) = self.generate_call_base_fn_parts(
            fn_decl,
            call_struct,
            base_fn_block,
            mock_type,
            Target::Other,
            None,
        );
        let impl_item_fn = ImplItemFn {
            attrs: Vec::new(),
            vis: Visibility::Inherited,
            defaultness: None,
            sig,
            block,
        };

        let base_fn = BaseFn { impl_item_fn };
        return base_fn;
    }

    fn generate_struct_trait_fn(
        &self,
        mock_type: &MockType,
        fn_decl: &FnDecl,
        call_struct: &CallStruct,
        base_fn_block: Block,
        trait_ident: &Ident,
    ) -> BaseFn {
        let (sig, block) = self.generate_call_base_fn_parts(
            fn_decl,
            call_struct,
            base_fn_block,
            mock_type,
            Target::Other,
            Some(trait_ident),
        );
        let impl_item_fn = ImplItemFn {
            attrs: Vec::new(),
            vis: Visibility::Inherited,
            defaultness: None,
            sig,
            block,
        };

        let base_fn = BaseFn { impl_item_fn };
        return base_fn;
    }

    fn generate_static(
        &self,
        mock_type: &MockType,
        fn_decl: &FnDecl,
        call_struct: &CallStruct,
        base_fn_block: Block,
    ) -> StaticBaseFn {
        let (sig, block) = self.generate_call_base_fn_parts(
            fn_decl,
            call_struct,
            base_fn_block,
            mock_type,
            Target::StaticFn,
            None,
        );
        let item_fn = ItemFn {
            attrs: Vec::new(),
            vis: Visibility::Inherited,
            sig,
            block: Box::new(block),
        };

        let static_base_fn = StaticBaseFn { item_fn };
        return static_base_fn;
    }
}

impl BaseFnGenerator {
    const CALL_ARG_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("call"));

    fn generate_call_base_fn_parts(
        &self,
        fn_decl: &FnDecl,
        call_struct: &CallStruct,
        base_fn_block: Block,
        mock_type: &MockType,
        target: Target,
        maybe_containing_trait_ident: Option<&Ident>,
    ) -> (Signature, Block) {
        let generics = match target {
            Target::StaticFn => fn_decl.merged_generics.clone(),
            Target::Other => fn_decl.own_generics.clone(),
        };
        let first_arg = match target {
            Target::StaticFn => FnArg::Typed(PatType {
                attrs: Vec::new(),
                pat: Box::new(Pat::Wild(PatWild {
                    attrs: Vec::new(),
                    underscore_token: Default::default(),
                })),
                colon_token: Default::default(),
                ty: Box::new(r#type::reference(mock_type.ty.clone(), None)),
            }),
            Target::Other => constants::REF_SELF_ARG.clone(),
        };
        let call_arg = FnArg::Typed(PatType {
            attrs: Vec::new(),
            pat: Box::new(Pat::Ident(PatIdent {
                attrs: Vec::new(),
                by_ref: None,
                mutability: None,
                ident: Self::CALL_ARG_IDENT.clone(),
                subpat: None,
            })),
            colon_token: Default::default(),
            ty: Box::new(call_struct.ty.clone()),
        });
        let ident =
            match maybe_containing_trait_ident {
                Some(containing_trait_ident) => self.base_fn_ident_formatter.format(
                    &format_ident!("{}_{}", containing_trait_ident, fn_decl.fn_ident),
                ),
                None => self.base_fn_ident_formatter.format(&fn_decl.fn_ident),
            };
        let inputs = [first_arg, call_arg];
        let sig = Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: Default::default(),
            ident,
            generics,
            paren_token: Default::default(),
            inputs: inputs.into_iter().collect(),
            variadic: None,
            output: fn_decl.return_value.clone(),
        };
        let block = self.generate_call_base_fn_block(
            fn_decl,
            call_struct,
            base_fn_block,
            mock_type.generics.get_phantom_fields_count(),
        );
        return (sig, block);
    }

    fn generate_call_base_fn_block(
        &self,
        fn_decl: &FnDecl,
        call_struct: &CallStruct,
        base_fn_block: Block,
        phantom_fields_count: usize,
    ) -> Block {
        let call_struct_fields = call_struct
            .item_struct
            .fields
            .iter()
            .skip(fn_decl.get_internal_phantom_types_count() + phantom_fields_count);
        let fields = fn_decl
            .arguments
            .iter()
            .filter_map(|fn_arg| match fn_arg {
                FnArg::Receiver(_) => None,
                FnArg::Typed(typed) => Some(typed),
            })
            .zip(call_struct_fields)
            .map(|(typed_fn_arg, call_struct_field)| FieldPat {
                attrs: Vec::new(),
                member: Member::Named(call_struct_field.get_required_ident()),
                colon_token: Some(Default::default()),
                pat: typed_fn_arg.pat.clone(),
            })
            .collect();
        let deconstruct_call_stmt = Stmt::Local(Local {
            attrs: Vec::new(),
            let_token: Default::default(),
            pat: Pat::Struct(PatStruct {
                attrs: Vec::new(),
                qself: None,
                path: path::create(call_struct.item_struct.ident.clone()),
                brace_token: Default::default(),
                fields,
                rest: Some(PatRest {
                    attrs: Vec::new(),
                    dot2_token: Default::default(),
                }),
            }),
            init: Some(LocalInit {
                eq_token: Default::default(),
                expr: Box::new(path::create_expr(Self::CALL_ARG_IDENT.clone())),
                diverge: None,
            }),
            semi_token: Default::default(),
        });
        let stmts = std::iter::once(deconstruct_call_stmt)
            .chain(base_fn_block.stmts)
            .collect();
        let block = Block {
            brace_token: Default::default(),
            stmts,
        };
        return block;
    }
}

enum Target {
    StaticFn,
    Other,
}
