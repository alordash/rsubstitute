use syn::*;

pub(crate) struct ReferenceToPointerConversionResult {
    pub new_type: Type,
    pub maybe_actual_source_type: Option<Type>
}