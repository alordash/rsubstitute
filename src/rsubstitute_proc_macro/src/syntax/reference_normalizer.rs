use crate::constants;
use syn::punctuated::Punctuated;
use syn::*;

pub trait IReferenceNormalizer {
    fn normalize_in_struct(&self, item_struct: &mut ItemStruct);

    fn normalize_in_impl(&self, lifetime: Lifetime, item_impl: &mut ItemImpl);
}

pub(crate) struct ReferenceNormalizer;

impl IReferenceNormalizer for ReferenceNormalizer {
    fn normalize_in_struct(&self, item_struct: &mut ItemStruct) {
        let fields_contain_reference_type: Vec<_> = item_struct
            .fields
            .iter_mut()
            .map(|field| self.process_and_check_if_contains_reference_type(&mut field.ty))
            .collect();
        let has_reference_types = fields_contain_reference_type
            .into_iter()
            .any(|contains_reference_type| contains_reference_type);
        if !has_reference_types {
            return;
        }

        // item_struct.generics.params.insert(
        //     0,
        //     GenericParam::Lifetime(LifetimeParam {
        //         attrs: Vec::new(),
        //         lifetime: constants::DEFAULT_ARG_FIELD_LIFETIME.clone(),
        //         colon_token: None,
        //         bounds: Punctuated::new(),
        //     }),
        // )
    }

    // TODO - just always add as first generic parameter
    fn normalize_in_impl(&self, lifetime: Lifetime, item_impl: &mut ItemImpl) {
        // item_impl.generics.params.insert(
        //     0,
        //     GenericParam::Lifetime(LifetimeParam {
        //         attrs: Vec::new(),
        //         lifetime: lifetime.clone(),
        //         colon_token: None,
        //         bounds: Punctuated::new(),
        //     }),
        // );
        if let Type::Path(type_path) = item_impl.self_ty.as_mut() {
            let last_segment = type_path
                .path
                .segments
                .last_mut()
                .expect("impl must have self_ty with non-empty path");
            let generic_argument = GenericArgument::Lifetime(lifetime);
            match &mut last_segment.arguments {
                PathArguments::AngleBracketed(angle_bracketed) => {
                    angle_bracketed.args.push(generic_argument)
                }
                _ => {
                    last_segment.arguments =
                        PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                            colon2_token: None,
                            lt_token: Default::default(),
                            args: [generic_argument].into_iter().collect(),
                            gt_token: Default::default(),
                        })
                }
            };
        }
    }
}

impl ReferenceNormalizer {
    fn process_and_check_if_contains_reference_type(&self, ty: &mut Type) -> bool {
        let result = match ty {
            Type::Array(type_array) => {
                self.process_and_check_if_contains_reference_type(type_array.elem.as_mut())
            }
            Type::Paren(type_paren) => {
                self.process_and_check_if_contains_reference_type(type_paren.elem.as_mut())
            }
            Type::Path(type_path) => {
                self.process_and_check_if_type_path_contains_reference_type(type_path)
            }
            Type::Reference(type_reference) => {
                type_reference.lifetime = Some(constants::DEFAULT_ARG_FIELD_LIFETIME.clone());
                self.process_and_check_if_contains_reference_type(type_reference.elem.as_mut());
                return true;
            }
            Type::Slice(type_slice) => {
                self.process_and_check_if_contains_reference_type(type_slice.elem.as_mut())
            }
            Type::Tuple(type_tuple) => {
                self.process_and_check_if_tuple_contains_reference_type(type_tuple)
            }
            _ => false,
        };
        return result;
    }

    fn process_and_check_if_type_path_contains_reference_type(
        &self,
        type_path: &mut TypePath,
    ) -> bool {
        let results: Vec<_> = type_path
            .path
            .segments
            .iter_mut()
            .flat_map(|path_segment| match &mut path_segment.arguments {
                PathArguments::AngleBracketed(angle_bracketed_generic_arguments) => {
                    let generic_arguments_results: Vec<_> = angle_bracketed_generic_arguments
                        .args
                        .iter_mut()
                        .flat_map(|generic_argument| match generic_argument {
                            GenericArgument::Type(ty) => {
                                Some(self.process_and_check_if_contains_reference_type(ty))
                            }
                            GenericArgument::AssocType(assoc_type) => {
                                Some(self.process_and_check_if_contains_reference_type(
                                    &mut assoc_type.ty,
                                ))
                            }
                            _ => None,
                        })
                        .collect();
                    let generic_arguments_result = generic_arguments_results
                        .into_iter()
                        .any(|contains_reference_type| contains_reference_type);
                    return Some(generic_arguments_result);
                }
                _ => None,
            })
            .collect();
        let result = results
            .into_iter()
            .any(|contains_reference_type| contains_reference_type);
        return result;
    }

    fn process_and_check_if_tuple_contains_reference_type(
        &self,
        type_tuple: &mut TypeTuple,
    ) -> bool {
        let results: Vec<_> = type_tuple
            .elems
            .iter_mut()
            .map(|elem_type| self.process_and_check_if_contains_reference_type(elem_type))
            .collect();
        let result = results
            .into_iter()
            .any(|contains_reference_type| contains_reference_type);
        return result;
    }
}
