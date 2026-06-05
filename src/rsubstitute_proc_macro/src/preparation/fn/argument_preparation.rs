use super::models::*;
use proc_macro2::Span;
use syn::visit_mut::{self, VisitMut};
use syn::*;

pub(crate) fn prepare_argument(pat_type: PatType) -> Argument {
    let mut inner = pat_type.clone();
    let outer = pat_type;
    ReferenceMutator.visit_type_mut(&mut inner.ty);
    let result = Argument { inner, outer };
    return result;
}

struct ReferenceMutator;

impl VisitMut for ReferenceMutator {
    fn visit_type_mut(&mut self, i: &mut Type) {
        let Type::Reference(i_ref) = i else {
            visit_mut::visit_type_mut(self, i);
            return;
        };
        let (const_token, mutability) = match i_ref.mutability {
            Some(x) => (None, Some(x)),
            None => (Some(Token![const](Span::call_site())), None),
        };

        // SAFETY: `empty_elem` is not used in any way except to mutate `Box<Type>` in place.
        let empty_elem = unsafe { Box::new_uninit().assume_init() };
        let elem = core::mem::replace(&mut i_ref.elem, empty_elem);
        let mut i_ptr = TypePtr {
            star_token: Token![*](i_ref.and_token.span),
            const_token,
            mutability,
            elem,
        };
        visit_mut::visit_type_ptr_mut(self, &mut i_ptr);
        *i = Type::Ptr(i_ptr);
    }
}
