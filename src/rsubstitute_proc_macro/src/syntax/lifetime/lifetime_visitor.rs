use syn::visit_mut::VisitMut;
use syn::*;

pub(crate) struct LifetimeReplacer<'a> {
    replacement_strategy: ReplacementStrategy<'a>,
}

impl<'a> LifetimeReplacer<'a> {
    pub(crate) fn new(
        replacement_strategy: ReplacementStrategy<'a>,
    ) -> Self {
        Self {
            replacement_strategy,
        }
    }
}

impl<'a> VisitMut for LifetimeReplacer<'a> {
    fn visit_lifetime_mut(&mut self, lifetime: &mut Lifetime) {
        match self.replacement_strategy {
            ReplacementStrategy::ReplaceAll(new_lifetime) => *lifetime = new_lifetime.clone(),
            ReplacementStrategy::ReplaceOnlyOptional(_) => {},
            ReplacementStrategy::RemoveAll => {}
        }

        visit_mut::visit_lifetime_mut(self, lifetime);
    }

    fn visit_type_reference_mut(&mut self, type_reference: &mut TypeReference) {
        match self.replacement_strategy {
            ReplacementStrategy::ReplaceAll(new_lifetime) => {
                type_reference.lifetime = Some(new_lifetime.clone())
            }
            ReplacementStrategy::ReplaceOnlyOptional(new_lifetime) if type_reference.lifetime.is_none() => {
                type_reference.lifetime = Some(new_lifetime.clone())
            }
            ReplacementStrategy::RemoveAll => type_reference.lifetime = None,
            _ => {}
        }

        visit_mut::visit_type_reference_mut(self, type_reference);
    }
}

pub(crate) enum ReplacementStrategy<'a> {
    ReplaceAll(&'a Lifetime),
    ReplaceOnlyOptional(&'a Lifetime),
    RemoveAll,
}
