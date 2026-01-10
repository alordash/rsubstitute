use syn::*;

pub(crate) enum LifetimeRef<'a> {
    Optional(&'a mut Option<Lifetime>),
    Required(&'a mut Lifetime),
}

impl<'a> LifetimeRef<'a> {
    pub fn set_lifetime(self, lifetime: Lifetime) {
        match self {
            LifetimeRef::Optional(optional_lifetime) => *optional_lifetime = Some(lifetime),
            LifetimeRef::Required(required_lifetime) => *required_lifetime = lifetime,
        }
    }
}
