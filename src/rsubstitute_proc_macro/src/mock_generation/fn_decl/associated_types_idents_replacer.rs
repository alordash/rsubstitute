use crate::constants;
use crate::syntax::r#type;
use proc_macro2::Ident;
use syn::punctuated::Punctuated;
use syn::visit_mut::VisitMut;
use syn::*;

pub(crate) struct AssociatedTypesIdentsReplacer {
    self_type_ident: Ident,
    new_path_segments: Punctuated<PathSegment, Token![::]>,
    qself: QSelf,
}

impl AssociatedTypesIdentsReplacer {
    pub fn new(mock_path: &Path, target_trait_path: &Path) -> Self {
        let new_path_segments = target_trait_path.segments.clone();
        let qself = QSelf {
            lt_token: Default::default(),
            ty: Box::new(r#type::create_from_path(mock_path.clone())),
            position: target_trait_path.segments.len(),
            as_token: Some(Default::default()),
            gt_token: Default::default(),
        };
        Self {
            self_type_ident: constants::SELF_TYPE_IDENT.clone(),
            qself,
            new_path_segments,
        }
    }
}

impl VisitMut for AssociatedTypesIdentsReplacer {
    fn visit_type_path_mut(&mut self, i: &mut TypePath) {
        if i.path.segments.len() > 1 && i.path.segments[0].ident == self.self_type_ident {
            let new_path_segments = self
                .new_path_segments
                .iter()
                .cloned()
                .chain(core::mem::take(&mut i.path.segments).into_iter().skip(1))
                .collect();
            i.path.segments = new_path_segments;
            i.qself = Some(self.qself.clone());
            return;
        }

        visit_mut::visit_type_path_mut(self, i);
    }
}
