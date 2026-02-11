use syn::*;

pub trait IReferenceTypeCrawler {
    fn get_all_optional_lifetimes<'a>(&self, ty: &'a mut Type) -> Vec<&'a mut Option<Lifetime>>;
}

pub struct ReferenceTypeCrawler;

impl IReferenceTypeCrawler for ReferenceTypeCrawler {
    fn get_all_optional_lifetimes<'a>(&self, ty: &'a mut Type) -> Vec<&'a mut Option<Lifetime>> {
        let mut result = Vec::new();
        self.recursive_get_all_type_references(&mut result, ty);
        return result;
    }
}

impl ReferenceTypeCrawler {
    fn recursive_get_all_type_references<'a>(
        &self,
        result: &mut Vec<&'a mut Option<Lifetime>>,
        ty: &'a mut Type,
    ) {
        match ty {
            Type::Array(type_array) => {
                self.recursive_get_all_type_references(result, type_array.elem.as_mut())
            }
            Type::Paren(type_paren) => {
                self.recursive_get_all_type_references(result, type_paren.elem.as_mut())
            }
            Type::Path(type_path) => {
                self.recursive_get_all_type_references_from_path(result, &mut type_path.path)
            }
            Type::Reference(type_reference) => {
                let mut_lifetime_ref = &mut type_reference.lifetime;
                self.recursive_get_all_type_references_from_box(result, &mut type_reference.elem);
                result.push(mut_lifetime_ref);
            }
            Type::Slice(type_slice) => {
                self.recursive_get_all_type_references(result, type_slice.elem.as_mut())
            }
            Type::Tuple(type_tuple) => {
                self.recursive_get_all_type_references_from_tuple(result, type_tuple)
            }
            Type::ImplTrait(type_impl_trait) => {
                self.recursive_get_all_type_references_from_type_impl_trait(result, type_impl_trait)
            }
            Type::Ptr(type_ptr) => {
                self.recursive_get_all_type_references(result, type_ptr.elem.as_mut())
            }
            _ => (),
        };
    }

    fn recursive_get_all_type_references_from_box<'a>(
        &self,
        result: &mut Vec<&'a mut Option<Lifetime>>,
        boxed_ty: &'a mut Box<Type>,
    ) {
        let ty = boxed_ty.as_mut();
        self.recursive_get_all_type_references(result, ty);
    }

    fn recursive_get_all_type_references_from_path<'a>(
        &self,
        result: &mut Vec<&'a mut Option<Lifetime>>,
        path: &'a mut Path,
    ) {
        for path_segment in path.segments.iter_mut() {
            match &mut path_segment.arguments {
                PathArguments::AngleBracketed(generic_arguments) => {
                    for generic_argument in generic_arguments.args.iter_mut() {
                        match generic_argument {
                            GenericArgument::Type(ty) => {
                                self.recursive_get_all_type_references(result, ty)
                            }
                            GenericArgument::AssocType(assoc_type) => {
                                self.recursive_get_all_type_references(result, &mut assoc_type.ty)
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
        &self,
        result: &mut Vec<&'a mut Option<Lifetime>>,
        type_tuple: &'a mut TypeTuple,
    ) {
        for elem_type in type_tuple.elems.iter_mut() {
            self.recursive_get_all_type_references(result, elem_type);
        }
    }

    fn recursive_get_all_type_references_from_type_impl_trait<'a>(
        &self,
        result: &mut Vec<&'a mut Option<Lifetime>>,
        type_impl_trait: &'a mut TypeImplTrait,
    ) {
        for type_param_bound in type_impl_trait.bounds.iter_mut() {
            match type_param_bound {
                TypeParamBound::Trait(r#trait) => {
                    self.recursive_get_all_type_references_from_path(result, &mut r#trait.path);
                }
                _ => (),
            }
        }
    }
}
