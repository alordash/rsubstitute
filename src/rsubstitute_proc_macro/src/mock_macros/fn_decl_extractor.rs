use crate::constants;
use crate::mock_macros::models::*;
use crate::syntax::*;
use std::sync::Arc;
use syn::*;

pub trait IFnDeclExtractor {
    fn extract(&self, items: &[TraitItem]) -> Vec<FnDecl>;

    fn extract_struct_fns(&self, impl_item_fns: &[&ImplItemFn]) -> Vec<FnDecl>;

    fn extract_struct_trait_impl_fns(&self, trait_impl: &TraitImpl) -> Vec<FnDecl>;

    fn extract_fn(&self, item_fn: &ItemFn) -> FnDecl;
}

pub(crate) struct FnDeclExtractor {
    pub arg_ident_extractor: Arc<dyn IArgIdentExtractor>,
}

impl IFnDeclExtractor for FnDeclExtractor {
    fn extract(&self, trait_items: &[TraitItem]) -> Vec<FnDecl> {
        let fn_decls = trait_items
            .into_iter()
            .flat_map(|x| self.try_map_trait_item_fn(x))
            .collect();
        return fn_decls;
    }

    fn extract_struct_fns(&self, impl_item_fns: &[&ImplItemFn]) -> Vec<FnDecl> {
        let fn_decls = impl_item_fns
            .iter()
            .filter(|impl_item_fn| impl_item_fn.sig.ident != constants::NEW_IDENT.clone())
            .map(|x| self.map_impl_item_fn(x))
            .collect();
        return fn_decls;
    }

    fn extract_struct_trait_impl_fns(&self, trait_impl: &TraitImpl) -> Vec<FnDecl> {
        let trait_ident = trait_impl.get_trait_ident_from_path();
        let fn_decls = trait_impl
            .get_fns()
            .iter()
            .map(move |trait_impl_fn| {
                self.create_fn_decl(
                    trait_impl_fn.attrs.clone(),
                    &trait_impl_fn.sig,
                    trait_impl_fn.vis.clone(),
                    Some(trait_impl_fn.block.clone()),
                    Some(trait_ident.clone()),
                )
            })
            .collect();
        return fn_decls;
    }

    fn extract_fn(&self, item_fn: &ItemFn) -> FnDecl {
        let fn_decl = self.create_fn_decl(
            item_fn.attrs.clone(),
            &item_fn.sig,
            item_fn.vis.clone(),
            Some(*item_fn.block.clone()),
            None,
        );
        return fn_decl;
    }
}

impl FnDeclExtractor {
    fn try_map_trait_item_fn(&self, trait_item: &TraitItem) -> Option<FnDecl> {
        match trait_item {
            TraitItem::Fn(trait_item_fn)
                if trait_item_fn.sig.ident != constants::NEW_IDENT.clone() =>
            {
                self.validate_signature(&trait_item_fn.sig);
                Some(self.map_trait_item_fn(trait_item_fn))
            }
            _ => None,
        }
    }

    fn map_trait_item_fn(&self, trait_item_fn: &TraitItemFn) -> FnDecl {
        let sig = &trait_item_fn.sig;
        let fn_decl = self.create_fn_decl(
            trait_item_fn.attrs.clone(),
            sig,
            Visibility::Inherited,
            trait_item_fn.default.clone(),
            None,
        );
        return fn_decl;
    }

    fn map_impl_item_fn(&self, impl_item_fn: &ImplItemFn) -> FnDecl {
        let sig = &impl_item_fn.sig;
        self.validate_signature(sig);
        let fn_decl = self.create_fn_decl(
            impl_item_fn.attrs.clone(),
            sig,
            impl_item_fn.vis.clone(),
            Some(impl_item_fn.block.clone()),
            None,
        );
        return fn_decl;
    }

    fn validate_signature(&self, sig: &Signature) {
        // TODO - remove? is it fully supported?
        // if !sig.generics.params.is_empty() {
        //     panic!("Generic type parameters for associated functions are not supported.");
        // }
    }

    fn create_fn_decl(
        &self,
        attrs: Vec<Attribute>,
        sig: &Signature,
        visibility: Visibility,
        maybe_base_fn_block: Option<Block>,
        maybe_parent_trait_ident: Option<Ident>,
    ) -> FnDecl {
        let has_phantom_return_type = self.is_return_type_generic(&sig.output, &sig.generics);
        let fn_decl = FnDecl {
            attrs,
            maybe_parent_trait_ident,
            fn_ident: sig.ident.clone(),
            arguments: sig
                .inputs
                .iter()
                .enumerate()
                .map(|(arg_number, fn_arg)| self.generate_fn_arg_info(arg_number, fn_arg))
                .collect(),
            return_value: sig.output.clone(),
            visibility,
            maybe_base_fn_block,
            base_callable: false, // TODO - set base callable properly (depending on argument in macro and if fn has base)
            has_phantom_return_type,
        };
        return fn_decl;
    }

    fn is_return_type_generic(&self, return_type: &ReturnType, generics: &Generics) -> bool {
        let ReturnType::Type(_, ty) = return_type else {
            return false;
        };
        let Type::Path(TypePath { path, .. }) = &**ty else {
            return false;
        };

        let type_params: Vec<_> = generics.type_params().collect();
        for segment in path.segments.iter() {
            if type_params
                .iter()
                .any(|type_param| segment.ident == type_param.ident)
            {
                return true;
            }
        }
        return false;
    }

    fn generate_fn_arg_info(&self, arg_number: usize, fn_arg: &FnArg) -> FnArg {
        match fn_arg {
            FnArg::Receiver(receiver) => FnArg::Receiver(receiver.clone()),
            FnArg::Typed(typed) => self.generate_typed_fn_arg(arg_number, typed),
        }
    }

    fn generate_typed_fn_arg(&self, arg_number: usize, typed: &PatType) -> FnArg {
        let arg_ident = self.arg_ident_extractor.extract(arg_number, typed);
        let fn_arg = FnArg::Typed(PatType {
            attrs: Vec::new(),
            pat: Box::new(Pat::Ident(PatIdent {
                attrs: Vec::new(),
                by_ref: None,
                mutability: None,
                ident: arg_ident,
                subpat: None,
            })),
            colon_token: Default::default(),
            ty: typed.ty.clone(),
        });
        return fn_arg;
    }

    // fn generate_fn_arg_info(&self, arg_number: usize, fn_arg: &FnArg) -> FnArgInfo {
    //     match fn_arg {
    //         FnArg::Receiver(receiver) => FnArgInfo::Receiver(receiver.clone()),
    //         FnArg::Typed(typed) => FnArgInfo::Typed(TypedFnArgInfo {
    //             ident: self.arg_ident_extractor.extract(arg_number, typed),
    //             ty: *typed.ty.clone(),
    //         }),
    //     }
    // }
}
