use syn::*;

pub trait IReferenceTypeCrawler {
    fn get_all_type_references<'a>(&self, ty: &'a mut Type) -> Vec<&'a mut TypeReference>;
}

pub struct ReferenceTypeCrawler;

impl IReferenceTypeCrawler for ReferenceTypeCrawler {
    fn get_all_type_references<'a>(&self, ty: &'a mut Type) -> Vec<&'a mut TypeReference> {
        let mut result = Vec::new();
        self.recursive_get_all_type_references(&mut result, ty);
        return result;
    }
}

impl ReferenceTypeCrawler {
    fn recursive_get_all_type_references<'a>(
        &self,
        result: &mut Vec<&'a mut TypeReference>,
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
                self.recursive_get_all_type_references_from_path(result, type_path)
            }
            Type::Reference(type_reference) => {
                // TODO - cursed. Maybe store only &mut Lifetime in result vector.
                let copy_mut_ref: &mut TypeReference = unsafe {
                    let ptr = type_reference as *mut TypeReference;
                    std::mem::transmute(ptr)
                };
                self.recursive_get_all_type_references_from_box(result, &mut type_reference.elem);
                result.push(copy_mut_ref);
            }
            Type::Slice(type_slice) => {
                self.recursive_get_all_type_references(result, type_slice.elem.as_mut())
            }
            Type::Tuple(type_tuple) => {
                self.recursive_get_all_type_references_from_tuple(result, type_tuple)
            }
            _ => (),
        };
    }

    fn recursive_get_all_type_references_from_box<'a>(
        &self,
        result: &mut Vec<&'a mut TypeReference>,
        boxed_ty: &'a mut Box<Type>,
    ) {
        let ty = boxed_ty.as_mut();
        self.recursive_get_all_type_references(result, ty);
    }

    fn recursive_get_all_type_references_from_path<'a>(
        &self,
        result: &mut Vec<&'a mut TypeReference>,
        type_path: &'a mut TypePath,
    ) {
        for path_segment in type_path.path.segments.iter_mut() {
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
        result: &mut Vec<&'a mut TypeReference>,
        type_tuple: &'a mut TypeTuple,
    ) {
        for elem_type in type_tuple.elems.iter_mut() {
            self.recursive_get_all_type_references(result, elem_type);
        }
    }
}
