use crate::constants;
use crate::syntax::r#type;
use proc_macro2::Ident;
use syn::visit_mut::VisitMut;
use syn::*;

pub(crate) struct AssociatedTypesIdentsReplacer {
    self_type_ident: Ident,
    new_path_segments: Vec<PathSegment>,
    qself: QSelf,
}

impl AssociatedTypesIdentsReplacer {
    pub fn new(mock_path: &Path, target_trait_path: &Path) -> Self {
        let new_path_segments = target_trait_path.segments.iter().cloned().collect();
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

    fn try_process_path(&self, path: &mut Path, qself: &mut Option<QSelf>) -> bool {
        if path.segments.len() <= 1 || path.segments[0].ident != self.self_type_ident {
            return false;
        }
        let new_path_segments = self
            .new_path_segments
            .iter()
            .cloned()
            .chain(core::mem::take(&mut path.segments).into_iter().skip(1))
            .collect();
        path.segments = new_path_segments;
        *qself = Some(self.qself.clone());
        return true;
    }
}

impl VisitMut for AssociatedTypesIdentsReplacer {
    fn visit_type_path_mut(&mut self, i: &mut TypePath) {
        if self.try_process_path(&mut i.path, &mut i.qself) {
            return;
        }

        visit_mut::visit_type_path_mut(self, i);
    }

    fn visit_expr_path_mut(&mut self, i: &mut PatPath) {
        if self.try_process_path(&mut i.path, &mut i.qself) {
            return;
        }

        visit_mut::visit_expr_path_mut(self, i);
    }
}
