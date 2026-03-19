use syn::visit_mut::VisitMut;
use syn::*;

pub(crate) fn convert_in_type(mut ty: Type) -> Type {
    let mut reference_to_pointer_convertor = ReferenceToPointerConvertor;
    reference_to_pointer_convertor.visit_type_mut(&mut ty);
    return ty;
}

struct ReferenceToPointerConvertor;

impl VisitMut for ReferenceToPointerConvertor {
    fn visit_type_mut(&mut self, i: &mut Type) {
        if let Type::Reference(type_reference) = i {
            let (const_token, mutability) = match type_reference.mutability {
                None => (Some(Default::default()), None),
                Some(mutability) => (None, Some(mutability)),
            };
            let mut type_ptr = TypePtr {
                star_token: Default::default(),
                const_token,
                mutability,
                elem: Box::new(Type::Never(TypeNever {
                    bang_token: Default::default(),
                })),
            };
            // TODO - is there a better way to do it?
            core::mem::swap(&mut type_ptr.elem, &mut type_reference.elem);
            *i = Type::Ptr(type_ptr);
        }

        visit_mut::visit_type_mut(self, i);
    }
}
