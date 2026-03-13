use syn::*;

pub(crate) fn recursive_visit_all_type_references<'a>(
    visitor: &mut dyn FnMut(&'a mut TypeReference),
    ty: &'a mut Type,
) {
    match ty {
        Type::Array(type_array) => {
            recursive_visit_all_type_references(visitor, type_array.elem.as_mut())
        }
        Type::Paren(type_paren) => {
            recursive_visit_all_type_references(visitor, type_paren.elem.as_mut())
        }
        Type::Path(type_path) => {
            recursive_get_all_type_references_from_path(visitor, &mut type_path.path)
        }
        Type::Reference(type_reference) => {
            // SAFETY: `recursive_get_all_type_references_from_box` does not modify received
            // reference, it needs mutable reference only to return it.
            let type_reference_clone = unsafe { core::mem::transmute_copy(&type_reference) };
            visitor(type_reference_clone);
            recursive_get_all_type_references_from_box(visitor, &mut type_reference.elem);
        }
        Type::Slice(type_slice) => {
            recursive_visit_all_type_references(visitor, type_slice.elem.as_mut())
        }
        Type::Tuple(type_tuple) => {
            recursive_get_all_type_references_from_tuple(visitor, type_tuple)
        }
        Type::ImplTrait(type_impl_trait) => {
            recursive_get_all_type_references_from_type_impl_trait(visitor, type_impl_trait)
        }
        Type::Ptr(type_ptr) => recursive_visit_all_type_references(visitor, type_ptr.elem.as_mut()),
        _ => (),
    };
}

fn recursive_get_all_type_references_from_box<'a>(
    visitor: &mut dyn FnMut(&'a mut TypeReference),
    boxed_ty: &'a mut Box<Type>,
) {
    let ty = boxed_ty.as_mut();
    recursive_visit_all_type_references(visitor, ty);
}

fn recursive_get_all_type_references_from_path<'a>(
    visitor: &mut dyn FnMut(&'a mut TypeReference),
    path: &'a mut Path,
) {
    for path_segment in path.segments.iter_mut() {
        match &mut path_segment.arguments {
            PathArguments::AngleBracketed(generic_arguments) => {
                for generic_argument in generic_arguments.args.iter_mut() {
                    match generic_argument {
                        GenericArgument::Type(ty) => {
                            recursive_visit_all_type_references(visitor, ty)
                        }
                        GenericArgument::AssocType(assoc_type) => {
                            recursive_visit_all_type_references(visitor, &mut assoc_type.ty)
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }
}

fn recursive_get_all_type_references_from_tuple<'a>(
    visitor: &mut dyn FnMut(&'a mut TypeReference),
    type_tuple: &'a mut TypeTuple,
) {
    for elem_type in type_tuple.elems.iter_mut() {
        recursive_visit_all_type_references(visitor, elem_type);
    }
}

fn recursive_get_all_type_references_from_type_impl_trait<'a>(
    visitor: &mut dyn FnMut(&'a mut TypeReference),
    type_impl_trait: &'a mut TypeImplTrait,
) {
    for type_param_bound in type_impl_trait.bounds.iter_mut() {
        match type_param_bound {
            TypeParamBound::Trait(r#trait) => {
                recursive_get_all_type_references_from_path(visitor, &mut r#trait.path);
            }
            _ => (),
        }
    }
}
