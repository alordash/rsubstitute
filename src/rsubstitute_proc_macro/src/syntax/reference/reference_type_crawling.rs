use super::*;
use syn::*;

pub(crate) fn recursive_get_all_type_references<'a>(
    result: &mut Vec<ReferenceLifetime<'a>>,
    ty: &'a mut Type,
) {
    match ty {
        Type::Array(type_array) => {
            recursive_get_all_type_references(result, type_array.elem.as_mut())
        }
        Type::Paren(type_paren) => {
            recursive_get_all_type_references(result, type_paren.elem.as_mut())
        }
        Type::Path(type_path) => {
            recursive_get_all_type_references_from_path(result, &mut type_path.path)
        }
        Type::Reference(type_reference) => {
            let mut_lifetime_ref = &mut type_reference.lifetime;
            recursive_get_all_type_references_from_box(result, &mut type_reference.elem);
            result.push(ReferenceLifetime::Optional(mut_lifetime_ref));
        }
        Type::Slice(type_slice) => {
            recursive_get_all_type_references(result, type_slice.elem.as_mut())
        }
        Type::Tuple(type_tuple) => recursive_get_all_type_references_from_tuple(result, type_tuple),
        Type::ImplTrait(type_impl_trait) => {
            recursive_get_all_type_references_from_type_impl_trait(result, type_impl_trait)
        }
        Type::Ptr(type_ptr) => recursive_get_all_type_references(result, type_ptr.elem.as_mut()),
        _ => (),
    };
}

pub(crate) fn recursive_get_all_type_references_from_box<'a>(
    result: &mut Vec<ReferenceLifetime<'a>>,
    boxed_ty: &'a mut Box<Type>,
) {
    let ty = boxed_ty.as_mut();
    recursive_get_all_type_references(result, ty);
}

pub(crate) fn recursive_get_all_type_references_from_path<'a>(
    result: &mut Vec<ReferenceLifetime<'a>>,
    path: &'a mut Path,
) {
    for path_segment in path.segments.iter_mut() {
        match &mut path_segment.arguments {
            PathArguments::AngleBracketed(generic_arguments) => {
                for generic_argument in generic_arguments.args.iter_mut() {
                    match generic_argument {
                        GenericArgument::Type(ty) => recursive_get_all_type_references(result, ty),
                        GenericArgument::AssocType(assoc_type) => {
                            recursive_get_all_type_references(result, &mut assoc_type.ty)
                        }
                        GenericArgument::Lifetime(lifetime) => {
                            result.push(ReferenceLifetime::Required(lifetime))
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }
}

pub(crate) fn recursive_get_all_type_references_from_tuple<'a>(
    result: &mut Vec<ReferenceLifetime<'a>>,
    type_tuple: &'a mut TypeTuple,
) {
    for elem_type in type_tuple.elems.iter_mut() {
        recursive_get_all_type_references(result, elem_type);
    }
}

pub(crate) fn recursive_get_all_type_references_from_type_impl_trait<'a>(
    result: &mut Vec<ReferenceLifetime<'a>>,
    type_impl_trait: &'a mut TypeImplTrait,
) {
    for type_param_bound in type_impl_trait.bounds.iter_mut() {
        match type_param_bound {
            TypeParamBound::Trait(r#trait) => {
                recursive_get_all_type_references_from_path(result, &mut r#trait.path);
            }
            _ => (),
        }
    }
}
