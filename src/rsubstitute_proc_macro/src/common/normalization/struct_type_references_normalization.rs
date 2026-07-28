use crate::syntax::*;
use quote::format_ident;
use syn::spanned::Spanned;
use syn::visit_mut::VisitMut;
use syn::*;

pub(crate) fn normalize_struct_type_references(
    mut impl_item: ImplItem,
    struct_path: &Path,
) -> ImplItem {
    let mock_struct_path = path::from_base_path_with_ident(
        &struct_path,
        format_ident!("{}Mock", path::last_ident(&struct_path)),
    );
    let mut normalizer = StructTypeReferencesNormalizer {
        struct_path,
        mock_struct_path,
    };
    normalizer.visit_impl_item_mut(&mut impl_item);
    return impl_item;
}

pub(crate) fn normalize_struct_type_references_in_impl_item_fn(
    impl_item_fn: &mut ImplItemFn,
    struct_path: &Path,
    mock_struct_path: Path,
) {
    let mut normalizer = StructTypeReferencesNormalizer {
        struct_path,
        mock_struct_path,
    };
    normalizer.visit_impl_item_fn_mut(impl_item_fn);
}

struct StructTypeReferencesNormalizer<'a> {
    pub struct_path: &'a Path,
    mock_struct_path: Path,
}

impl<'a> StructTypeReferencesNormalizer<'a> {
    fn try_replace_path(&self, path: &mut Path) -> Option<Path> {
        if path
            .segments
            .first()
            .is_some_and(|first| first.ident == "Self")
        {
            let source_path = path.clone();
            let mut new_path = self.mock_struct_path.clone();
            new_path.segments = new_path
                .segments
                .into_iter()
                .chain(core::mem::take(&mut path.segments).into_iter().skip(1))
                .collect();
            *path = new_path;
            return Some(source_path);
        }
        if path::starts_with(path, self.struct_path) {
            let source_path = path.clone();
            let mut new_path = self.mock_struct_path.clone();
            new_path.segments = new_path
                .segments
                .into_iter()
                .chain(
                    core::mem::take(&mut path.segments)
                        .into_iter()
                        .skip(self.mock_struct_path.segments.len()),
                )
                .collect();
            *path = new_path;
            return Some(source_path);
        }
        return None;
    }
}

impl<'a> VisitMut for StructTypeReferencesNormalizer<'a> {
    fn visit_path_mut(&mut self, i: &mut Path) {
        self.try_replace_path(i);

        visit_mut::visit_path_mut(self, i);
    }
}
