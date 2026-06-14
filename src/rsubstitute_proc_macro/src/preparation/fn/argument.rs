use super::models::*;
use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::visit_mut::{self, VisitMut};
use syn::*;

pub(crate) fn new((number, pat_type): (usize, PatType)) -> Argument {
    let ident = prepare_ident(number, &pat_type);
    let mut inner_type = pat_type.ty.clone();
    replace_refs_with_ptrs(&mut inner_type);
    let result = Argument {
        pat_type,
        ident,
        inner_type,
    };
    return result;
}

fn prepare_ident(number: usize, pat_type: &PatType) -> Ident {
    let result = match pat_type.pat.as_ref() {
        Pat::Ident(pat_ident) => pat_ident.ident.clone(),
        not_ident => Ident::new(&format!("__pat_arg{number}"), not_ident.span()),
    };

    return result;
}

fn replace_refs_with_ptrs(ty: &mut Type) {
    ReferenceMutator.visit_type_mut(ty);

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
}
