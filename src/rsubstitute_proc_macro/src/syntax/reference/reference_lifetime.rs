use syn::Lifetime;

pub(crate) enum ReferenceLifetime<'a> {
    Optional(&'a mut Option<Lifetime>),
    Required(&'a mut Lifetime),
}

impl<'a> ReferenceLifetime<'a> {
    pub(crate) fn is_none(&self) -> bool {
        match self {
            ReferenceLifetime::Optional(optional) => optional.is_none(),
            ReferenceLifetime::Required(_) => false,
        }
    }

    pub(crate) fn get_mut(&mut self) -> Option<&mut Lifetime> {
        match self {
            ReferenceLifetime::Optional(optional) => optional.as_mut(),
            ReferenceLifetime::Required(required) => Some(*required),
        }
    }

    pub(crate) fn set(&mut self, lifetime: Lifetime) {
        match self {
            ReferenceLifetime::Optional(optional) => **optional = Some(lifetime),
            ReferenceLifetime::Required(required) => **required = lifetime,
        }
    }
}
