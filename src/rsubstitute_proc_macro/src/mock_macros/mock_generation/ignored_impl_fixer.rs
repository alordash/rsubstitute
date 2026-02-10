use crate::mock_macros::mock_generation::models::*;
use crate::syntax::*;
use std::sync::Arc;
use syn::*;

pub trait IIgnoredImplFixer {
    fn fix(&self, mock_type: &MockType, ignored_impls: &mut [ItemImpl]);
}

pub(crate) struct IgnoredImplFixer {
    pub generics_merger: Arc<dyn IGenericsMerger>,
}

impl IIgnoredImplFixer for IgnoredImplFixer {
    fn fix(&self, mock_type: &MockType, ignored_impls: &mut [ItemImpl]) {
        for ignored_impl in ignored_impls.iter_mut() {
            self.fix_single(mock_type, ignored_impl);
        }
    }
}

impl IgnoredImplFixer {
    fn fix_single(&self, mock_type: &MockType, ignored_impl: &mut ItemImpl) {
        let merged_generics = self
            .generics_merger
            .merge(&mock_type.generics.impl_generics, &ignored_impl.generics);
        ignored_impl.generics = merged_generics;
        *ignored_impl.self_ty = mock_type.ty.clone();
    }
}
