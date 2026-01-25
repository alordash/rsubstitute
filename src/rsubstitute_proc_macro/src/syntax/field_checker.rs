use crate::constants;
use syn::*;

pub trait IFieldChecker {
    fn is_phantom_data(&self, field: &Field) -> bool;
}

pub(crate) struct FieldChecker;

impl IFieldChecker for FieldChecker {
    fn is_phantom_data(&self, field: &Field) -> bool {
        if let Type::Path(type_path) = &field.ty
            && let Some(first_path_segment) = type_path.path.segments.first()
            && first_path_segment.ident == constants::PHANTOM_DATA_IDENT.clone()
        {
            return true;
        }
        return false;
    }
}
