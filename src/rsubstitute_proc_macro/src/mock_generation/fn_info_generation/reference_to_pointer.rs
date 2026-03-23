use crate::mock_generation::fn_info_generation::models::*;
use syn::visit_mut::VisitMut;
use syn::*;

pub(crate) fn convert_in_type(
    mut ty: Type,
    conversion_strategy: ConversionStrategy,
) -> ReferenceToPointerConversionResult {
    let mut reference_to_pointer_convertor = ReferenceToPointerConvertor::new(conversion_strategy);
    let actual_source_type = ty.clone();
    reference_to_pointer_convertor.visit_type_mut(&mut ty);
    let maybe_actual_source_type = if reference_to_pointer_convertor.changed {
        Some(actual_source_type)
    } else {
        None
    };
    return ReferenceToPointerConversionResult {
        new_type: ty,
        maybe_actual_source_type,
    };
}

pub enum ConversionStrategy {
    AllReferences,
    OnlyAnonymous,
}

impl ConversionStrategy {
    fn can_convert(&self, type_reference: &TypeReference) -> bool {
        match self {
            ConversionStrategy::AllReferences => true,
            ConversionStrategy::OnlyAnonymous => type_reference.lifetime.is_none(),
        }
    }
}

struct ReferenceToPointerConvertor {
    conversion_strategy: ConversionStrategy,
    changed: bool,
}

impl ReferenceToPointerConvertor {
    pub fn new(conversion_strategy: ConversionStrategy) -> Self {
        Self {
            conversion_strategy,
            changed: false,
        }
    }
}

impl VisitMut for ReferenceToPointerConvertor {
    fn visit_type_mut(&mut self, i: &mut Type) {
        if let Type::Reference(type_reference) = i
            && self.conversion_strategy.can_convert(type_reference)
        {
            let (const_token, mutability) = match type_reference.mutability {
                None => (Some(Default::default()), None),
                Some(mutability) => (None, Some(mutability)),
            };
            let mut type_ptr = TypePtr {
                star_token: Default::default(),
                const_token,
                mutability,
                elem: Box::new(Type::Never(TypeNever {
                    bang_token: Default::default(),
                })),
            };
            // TODO - is there a better way to do it?
            core::mem::swap(&mut type_ptr.elem, &mut type_reference.elem);
            *i = Type::Ptr(type_ptr);
            self.changed = true;
        }

        visit_mut::visit_type_mut(self, i);
    }
}
