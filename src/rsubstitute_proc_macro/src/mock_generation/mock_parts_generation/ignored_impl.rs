use crate::mock_generation::mock_parts_generation::models::*;
use crate::syntax::*;
use syn::*;

pub(crate) fn fix(mock_type: &MockType, ignored_impls: &mut [ItemImpl]) {
    for ignored_impl in ignored_impls.iter_mut() {
        fix_single(mock_type, ignored_impl);
    }
}

fn fix_single(mock_type: &MockType, ignored_impl: &mut ItemImpl) {
    let merged_generics = generics::merge(
        &mock_type.generics.impl_generics_without_default_values,
        &ignored_impl.generics,
    );
    ignored_impl.generics = merged_generics;
    *ignored_impl.self_ty = mock_type.ty.clone();
}
