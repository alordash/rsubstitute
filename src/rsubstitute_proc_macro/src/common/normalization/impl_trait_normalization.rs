use crate::syntax::*;
use syn::spanned::Spanned;
use syn::visit_mut::VisitMut;
use syn::*;

pub(crate) struct Result {
    pub ty: Type,
    pub is_impl_trait: bool,
}
pub(crate) fn replace_impl_trait_with_box_dyn_trait(mut ty: Type) -> Result {
    let mut impl_trait_replacer = ImplTraitReplacer::new();
    impl_trait_replacer.visit_type_mut(&mut ty);
    let result = Result {
        ty,
        is_impl_trait: impl_trait_replacer.is_impl_trait,
    };
    return result;
}

struct ImplTraitReplacer {
    pub is_impl_trait: bool,
}

impl ImplTraitReplacer {
    pub fn new() -> Self {
        Self {
            is_impl_trait: false,
        }
    }
}

impl VisitMut for ImplTraitReplacer {
    fn visit_type_mut(&mut self, i: &mut Type) {
        if let Type::ImplTrait(type_impl_trait) = i {
            let span = type_impl_trait.span();
            let type_dyn_trait = TypeTraitObject {
                attrs: Vec::new(),
                dyn_token: Some(Token![dyn](span)),
                bounds: core::mem::take(&mut type_impl_trait.bounds),
            };
            let type_box_dyn_trait = TypePath {
                attrs: Vec::new(),
                qself: None,
                path: path::new_generics(
                    span,
                    ["Box"],
                    [GenericArgument::Type(Type::TraitObject(type_dyn_trait))],
                ),
            };
            *i = Type::Path(type_box_dyn_trait);
            self.is_impl_trait = true;
        }

        visit_mut::visit_type_mut(self, i);
    }
}
