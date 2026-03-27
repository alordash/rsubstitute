use crate::constants;
use crate::mock_generation::fn_decl::associated_types_idents_replacer::AssociatedTypesIdentsReplacer;
use crate::mock_generation::mock_parts_generation::models::*;
use crate::mock_generation::models::*;
use crate::mock_generation::*;
use crate::syntax::*;
use syn::visit_mut::VisitMut;
use syn::*;

mod associated_types_idents_replacer;

pub(crate) fn extract(ctx: &Ctx, mock_type: &MockType, item_trait: &ItemTrait) -> Vec<FnDecl> {
    let fn_decls = item_trait
        .items
        .iter()
        .flat_map(|x| {
            try_map_trait_item_fn(ctx, mock_type, x, &path::create(item_trait.ident.clone()))
        })
        .collect();
    return fn_decls;
}

pub(crate) fn extract_struct_fns(
    ctx: &Ctx,
    mock_type: &MockType,
    impl_item_fns: &[&ImplItemFn],
) -> Vec<FnDecl> {
    let fn_decls = impl_item_fns
        .iter()
        .filter(|impl_item_fn| impl_item_fn.sig.ident != constants::NEW_IDENT.clone())
        .map(|x| map_impl_item_fn(ctx, mock_type, x))
        .collect();
    return fn_decls;
}

pub(crate) fn extract_struct_trait_impl_fns(
    ctx: &Ctx,
    mock_type: &MockType,
    trait_impl: &TraitImpl,
) -> Vec<FnDecl> {
    let fn_decls = trait_impl
        .get_fns()
        .iter()
        .map(move |trait_impl_fn| {
            create_fn_decl(
                ctx,
                mock_type,
                GenericsStrategy::MergeWithMockGenerics,
                trait_impl_fn.attrs.clone(),
                &trait_impl_fn.sig,
                trait_impl_fn.vis.clone(),
                false,
                Some(trait_impl_fn.block.clone()),
                Some(&trait_impl.trait_path),
                Some(&trait_impl.item_impl.generics),
            )
        })
        .collect();
    return fn_decls;
}

pub(crate) fn extract_fn(ctx: &Ctx, mock_type: &MockType, item_fn: &ItemFn) -> FnDecl {
    let fn_decl = create_fn_decl(
        ctx,
        mock_type,
        GenericsStrategy::UseMockGenerics,
        item_fn.attrs.clone(),
        &item_fn.sig,
        item_fn.vis.clone(),
        false,
        Some(*item_fn.block.clone()),
        None,
        None,
    );
    return fn_decl;
}

fn try_map_trait_item_fn(
    ctx: &Ctx,
    mock_type: &MockType,
    trait_item: &TraitItem,
    trait_path: &Path,
) -> Option<FnDecl> {
    match trait_item {
        TraitItem::Fn(trait_item_fn) if trait_item_fn.sig.ident != constants::NEW_IDENT.clone() => {
            Some(map_trait_item_fn(ctx, mock_type, trait_item_fn, trait_path))
        }
        _ => None,
    }
}

fn map_trait_item_fn(
    ctx: &Ctx,
    mock_type: &MockType,
    trait_item_fn: &TraitItemFn,
    trait_path: &Path,
) -> FnDecl {
    let sig = &trait_item_fn.sig;
    let fn_decl = create_fn_decl(
        ctx,
        mock_type,
        GenericsStrategy::MergeWithMockGenerics,
        trait_item_fn.attrs.clone(),
        sig,
        Visibility::Inherited,
        true,
        trait_item_fn.default.clone(),
        Some(trait_path),
        None,
    );
    return fn_decl;
}

fn map_impl_item_fn(ctx: &Ctx, mock_type: &MockType, impl_item_fn: &ImplItemFn) -> FnDecl {
    let sig = &impl_item_fn.sig;
    let fn_decl = create_fn_decl(
        ctx,
        mock_type,
        GenericsStrategy::MergeWithMockGenerics,
        impl_item_fn.attrs.clone(),
        sig,
        impl_item_fn.vis.clone(),
        false,
        Some(impl_item_fn.block.clone()),
        None,
        None,
    );
    return fn_decl;
}

fn create_fn_decl(
    ctx: &Ctx,
    mock_type: &MockType,
    generics_strategy: GenericsStrategy,
    attrs: Vec<Attribute>,
    sig: &Signature,
    visibility: Visibility,
    base_fn_block_is_in_trait: bool,
    mut maybe_base_fn_block: Option<Block>,
    maybe_parent_trait_path: Option<&Path>,
    maybe_struct_parent_trait_generics: Option<&Generics>,
) -> FnDecl {
    let mut actual_sig = sig.clone();
    if let Some(parent_trait_path) = maybe_parent_trait_path {
        let mock_path = &path::create_with_generics(
            mock_type.ident.clone(),
            mock_type.generics.impl_generics.clone(),
        );
        let mut associated_types_idents_replacer =
            AssociatedTypesIdentsReplacer::new(mock_path, parent_trait_path);

        associated_types_idents_replacer.visit_signature_mut(&mut actual_sig);
        if base_fn_block_is_in_trait {
            maybe_base_fn_block = maybe_base_fn_block.map(|mut block| {
                associated_types_idents_replacer.visit_block_mut(&mut block);
                return block;
            });
        }
    }

    let maybe_phantom_return_field =
        try_get_phantom_return_field(&actual_sig.output, &actual_sig.generics);
    let mut merged_generics = match generics_strategy {
        GenericsStrategy::MergeWithMockGenerics => generics::merge(
            mock_type.generics.impl_generics.clone(),
            &actual_sig.generics,
        ),
        GenericsStrategy::UseMockGenerics => mock_type.generics.impl_generics.clone(),
    };
    if let Some(struct_parent_trait_generics) = maybe_struct_parent_trait_generics {
        merged_generics = generics::merge(merged_generics, struct_parent_trait_generics);
    }
    let arguments: Vec<_> = actual_sig.inputs.iter().cloned().collect();
    let arg_refs_tuple = generate_arg_refs_tuple(&arguments);
    let internal_phantom_fields: Vec<_> = actual_sig
        .inputs
        .iter()
        .enumerate()
        .flat_map(|(arg_number, fn_arg)| phantom_field::try_create_for_fn_arg(arg_number, fn_arg))
        .chain(
            actual_sig
                .generics
                .params
                .iter()
                .filter_map(phantom_field::try_map_generic_param),
        )
        .chain(maybe_phantom_return_field)
        .collect();
    let maybe_actual_self_type = try_get_actual_self_type(&actual_sig, &mock_type.ty_path);

    let fn_decl = FnDecl {
        attrs,
        maybe_parent_trait_ident: maybe_parent_trait_path.map(ident::flatten_path_to_ident),
        fn_ident: actual_sig.ident.clone(),
        arguments,
        return_value: actual_sig.output.clone(),
        own_generics: actual_sig.generics.clone(),
        merged_generics,
        visibility,
        maybe_base_fn_block: maybe_base_fn_block.filter(|_| ctx.support_base_calling),
        internal_phantom_fields,
        arg_refs_tuple,
        maybe_actual_self_type,
    };
    return fn_decl;
}

fn try_get_phantom_return_field(return_type: &ReturnType, generics: &Generics) -> Option<Field> {
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

fn generate_arg_refs_tuple(fn_args: &[FnArg]) -> Type {
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

fn try_get_actual_self_type(sig: &Signature, inferred_type_path: &TypePath) -> Option<Receiver> {
    let Some(FnArg::Receiver(mut receiver)) = sig.inputs.get(0).cloned() else {
        return None;
    };
    receiver.colon_token = Some(Default::default());
    receiver.reference = None;
    let mut self_type_inferrer = SelfTypeInferrer { inferred_type_path };
    self_type_inferrer.visit_receiver_mut(&mut receiver);
    return Some(receiver.clone());
}

fn is_type_path_receiver_self(ty_path: &TypePath) -> bool {
    ty_path
        .path
        .get_ident()
        .is_some_and(|ident| ident == constants::SELF_TYPE_NAME)
}

enum GenericsStrategy {
    MergeWithMockGenerics,
    UseMockGenerics,
}

struct SelfTypeInferrer<'a> {
    pub inferred_type_path: &'a TypePath,
}

impl<'a> VisitMut for SelfTypeInferrer<'a> {
    fn visit_type_path_mut(&mut self, i: &mut TypePath) {
        if is_type_path_receiver_self(i) {
            *i = self.inferred_type_path.clone();
            return;
        }

        visit_mut::visit_type_path_mut(self, i);
    }
}
