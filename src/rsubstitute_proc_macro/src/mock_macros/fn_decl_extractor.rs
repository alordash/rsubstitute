use crate::constants;
use crate::mock_macros::mock_generation::models::*;
use crate::mock_macros::models::*;
use crate::syntax::*;
use syn::*;

pub(crate) trait IFnDeclExtractor {
    fn extract(&self, ctx: &Ctx, mock_generics: &MockGenerics, items: &[TraitItem]) -> Vec<FnDecl>;

    fn extract_struct_fns(
        &self,
        ctx: &Ctx,
        mock_generics: &MockGenerics,
        impl_item_fns: &[&ImplItemFn],
    ) -> Vec<FnDecl>;

    fn extract_struct_trait_impl_fns(
        &self,
        ctx: &Ctx,
        mock_generics: &MockGenerics,
        trait_impl: &TraitImpl,
    ) -> Vec<FnDecl>;

    fn extract_fn(&self, ctx: &Ctx, mock_generics: &MockGenerics, item_fn: &ItemFn) -> FnDecl;
}

pub(crate) struct FnDeclExtractor;

impl IFnDeclExtractor for FnDeclExtractor {
    fn extract(
        &self,
        ctx: &Ctx,
        mock_generics: &MockGenerics,
        trait_items: &[TraitItem],
    ) -> Vec<FnDecl> {
        let fn_decls = trait_items
            .into_iter()
            .flat_map(|x| self.try_map_trait_item_fn(ctx, mock_generics, x))
            .collect();
        return fn_decls;
    }

    fn extract_struct_fns(
        &self,
        ctx: &Ctx,
        mock_generics: &MockGenerics,
        impl_item_fns: &[&ImplItemFn],
    ) -> Vec<FnDecl> {
        let fn_decls = impl_item_fns
            .iter()
            .filter(|impl_item_fn| impl_item_fn.sig.ident != constants::NEW_IDENT.clone())
            .map(|x| self.map_impl_item_fn(ctx, mock_generics, x))
            .collect();
        return fn_decls;
    }

    fn extract_struct_trait_impl_fns(
        &self,
        ctx: &Ctx,
        mock_generics: &MockGenerics,
        trait_impl: &TraitImpl,
    ) -> Vec<FnDecl> {
        let trait_ident = trait_impl.get_trait_ident_from_path();
        let fn_decls = trait_impl
            .get_fns()
            .iter()
            .map(move |trait_impl_fn| {
                self.create_fn_decl(
                    ctx,
                    GenericsStrategy::MergeWithMockGenerics(mock_generics),
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

    fn extract_fn(&self, ctx: &Ctx, mock_generics: &MockGenerics, item_fn: &ItemFn) -> FnDecl {
        let fn_decl = self.create_fn_decl(
            ctx,
            GenericsStrategy::UseMockGenerics(mock_generics),
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
    fn try_map_trait_item_fn(
        &self,
        ctx: &Ctx,
        mock_generics: &MockGenerics,
        trait_item: &TraitItem,
    ) -> Option<FnDecl> {
        match trait_item {
            TraitItem::Fn(trait_item_fn)
                if trait_item_fn.sig.ident != constants::NEW_IDENT.clone() =>
            {
                Some(self.map_trait_item_fn(ctx, mock_generics, trait_item_fn))
            }
            _ => None,
        }
    }

    fn map_trait_item_fn(
        &self,
        ctx: &Ctx,
        mock_generics: &MockGenerics,
        trait_item_fn: &TraitItemFn,
    ) -> FnDecl {
        let sig = &trait_item_fn.sig;
        let fn_decl = self.create_fn_decl(
            ctx,
            GenericsStrategy::MergeWithMockGenerics(mock_generics),
            trait_item_fn.attrs.clone(),
            sig,
            Visibility::Inherited,
            trait_item_fn.default.clone(),
            None,
        );
        return fn_decl;
    }

    fn map_impl_item_fn(
        &self,
        ctx: &Ctx,
        mock_generics: &MockGenerics,
        impl_item_fn: &ImplItemFn,
    ) -> FnDecl {
        let sig = &impl_item_fn.sig;
        let fn_decl = self.create_fn_decl(
            ctx,
            GenericsStrategy::MergeWithMockGenerics(mock_generics),
            impl_item_fn.attrs.clone(),
            sig,
            impl_item_fn.vis.clone(),
            Some(impl_item_fn.block.clone()),
            None,
        );
        return fn_decl;
    }

    fn create_fn_decl(
        &self,
        ctx: &Ctx,
        generics_strategy: GenericsStrategy,
        attrs: Vec<Attribute>,
        sig: &Signature,
        visibility: Visibility,
        maybe_base_fn_block: Option<Block>,
        maybe_parent_trait_ident: Option<Ident>,
    ) -> FnDecl {
        let maybe_phantom_return_field =
            self.try_get_phantom_return_field(&sig.output, &sig.generics);
        let merged_generics = match generics_strategy {
            GenericsStrategy::MergeWithMockGenerics(mock_generics) => {
                generics::merge(&mock_generics.impl_generics, &sig.generics)
            }
            GenericsStrategy::UseMockGenerics(mock_generics) => mock_generics.impl_generics.clone(),
        };
        let arguments: Vec<_> = sig.inputs.iter().cloned().collect();
        let arg_refs_tuple = self.generate_arg_refs_tuple(&arguments);
        let fn_decl = FnDecl {
            attrs,
            maybe_parent_trait_ident,
            fn_ident: sig.ident.clone(),
            arguments,
            return_value: sig.output.clone(),
            own_generics: sig.generics.clone(),
            merged_generics,
            visibility,
            maybe_base_fn_block: maybe_base_fn_block.filter(|_| ctx.support_base_calling),
            maybe_phantom_return_field,
            arg_refs_tuple,
        };
        return fn_decl;
    }

    fn try_get_phantom_return_field(
        &self,
        return_type: &ReturnType,
        generics: &Generics,
    ) -> Option<Field> {
        let ReturnType::Type(_, ty) = return_type else {
            return None;
        };
        let Type::Path(TypePath { path, .. }) = &**ty else {
            return None;
        };
        let last_segment = path.segments.last()?;

        let type_params: Vec<_> = generics.type_params().collect();
        if type_params
            .iter()
            .any(|type_param| last_segment.ident == type_param.ident)
        {
            let field = field::create(
                constants::RETURN_TYPE_PHANTOM_FIELD_IDENT.clone(),
                r#type::phantom_data(last_segment.ident.clone()),
            );
            return Some(field);
        }
        return None;
    }

    fn generate_arg_refs_tuple(&self, fn_args: &[FnArg]) -> Type {
        let result = Type::Tuple(TypeTuple {
            paren_token: Default::default(),
            elems: fn_args
                .iter()
                .filter_map(|fn_arg| match fn_arg {
                    FnArg::Receiver(_) => None,
                    FnArg::Typed(pat_type) => Some(*pat_type.ty.clone()),
                })
                .map(|ty| r#type::reference(ty, None))
                .collect(),
        });
        return result;
    }
}

enum GenericsStrategy<'a> {
    MergeWithMockGenerics(&'a MockGenerics),
    UseMockGenerics(&'a MockGenerics),
}
