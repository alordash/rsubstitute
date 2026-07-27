use crate::common::{data_field, generics_field, mockable_field};
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
    fn visit_expr_struct_mut(&mut self, i: &mut ExprStruct) {
        if let Some(source_path) = self.try_replace_path(&mut i.path) {
            let mut source_expr_struct = i.clone();
            let span = source_expr_struct.span();
            source_expr_struct.path = source_path;
            i.fields = punctuated([
                generics_field::new_value(span),
                data_field::new_clone_value(span),
                mockable_field::new_value(span, Expr::Struct(source_expr_struct)),
            ]);
        }

        visit_mut::visit_expr_struct_mut(self, i);
    }

    fn visit_pat_struct_mut(&mut self, i: &mut PatStruct) {
        if let Some(source_path) = self.try_replace_path(&mut i.path) {
            // TODO - limitation: can not deconstruct mocked struct in mocked impls
            // because deconstructed value can either be `Struct` or `StructMock`
            // and there is no way to distinguish them without running code analysis
        }

        visit_mut::visit_pat_struct_mut(self, i);
    }

    fn visit_path_mut(&mut self, i: &mut Path) {
        self.try_replace_path(i);

        visit_mut::visit_path_mut(self, i);
    }
}

struct SHolder {
    pub sb: Box<S>,
}
struct S {
    pub n: i32,
}

fn q(sholder: SHolder) {
    let SHolder { sb: S { n } } = sholder;
}
