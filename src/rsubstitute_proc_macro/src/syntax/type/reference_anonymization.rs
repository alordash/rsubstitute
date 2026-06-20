use crate::generation::rsubstitute_lifetime;
use syn::spanned::Spanned;
use syn::visit_mut::*;
use syn::*;

pub(crate) fn anonymize_all_references(mut ty: Type) -> Type {
    ReferenceAnonymizer.visit_type_mut(&mut ty);
    return ty;
}

struct ReferenceAnonymizer;

impl VisitMut for ReferenceAnonymizer {
    fn visit_type_reference_mut(&mut self, i: &mut TypeReference) {
        i.lifetime = Some(rsubstitute_lifetime::new(i.span()));
        visit_mut::visit_type_reference_mut(self, i);
    }
}
