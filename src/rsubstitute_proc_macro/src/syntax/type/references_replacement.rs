use crate::syntax::*;
use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::visit_mut::VisitMut;
use syn::*;

pub(crate) fn replace_anonymous_lifetimes_in_references(
    mut ty: Box<Type>,
    replacement: &Lifetime,
) -> Box<Type> {
    AnonymousLifetimeReplacer { replacement }.visit_type_mut(&mut ty);
    return ty;
}

struct AnonymousLifetimeReplacer<'a> {
    replacement: &'a Lifetime,
}

impl<'a> VisitMut for AnonymousLifetimeReplacer<'a> {
    fn visit_type_reference_mut(&mut self, i: &mut TypeReference) {
        if i.lifetime.is_none() {
            i.lifetime = Some(self.replacement.clone());
        }
        visit_mut::visit_type_reference_mut(self, i);
    }
}

pub(crate) fn replace_references_with_pointers(mut ty: Box<Type>) -> Box<Type> {
    ReferenceToPointerConverter.visit_type_mut(&mut ty);
    return ty;
}

struct ReferenceToPointerConverter;

impl VisitMut for ReferenceToPointerConverter {
    fn visit_type_mut(&mut self, i: &mut Type) {
        let Type::Reference(i_ref) = i else {
            visit_mut::visit_type_mut(self, i);
            return;
        };
        let (const_token, mutability) = match i_ref.mutability {
            Some(x) => (None, Some(x)),
            None => (Some(Token![const](Span::call_site())), None),
        };

        let void_elem = Box::new(void_type(i_ref.span()));
        let elem = core::mem::replace(&mut i_ref.elem, void_elem);
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
