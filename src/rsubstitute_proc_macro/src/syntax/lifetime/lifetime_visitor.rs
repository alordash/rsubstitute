use syn::visit_mut::VisitMut;
use syn::*;

pub(crate) struct LifetimeReplacer<'a> {
    new_lifetime: &'a Lifetime,
    replacement_strategy: ReplacementStrategy,
}

impl<'a> LifetimeReplacer<'a> {
    pub(crate) fn new(
        new_lifetime: &'a Lifetime,
        replacement_strategy: ReplacementStrategy,
    ) -> Self {
        Self {
            new_lifetime,
            replacement_strategy,
        }
    }
}

impl<'a> VisitMut for LifetimeReplacer<'a> {
    fn visit_lifetime_mut(&mut self, lifetime: &mut Lifetime) {
        match self.replacement_strategy {
            ReplacementStrategy::ReplaceAll => *lifetime = self.new_lifetime.clone(),
            ReplacementStrategy::ReplaceOnlyOptional => {}
        }

        visit_mut::visit_lifetime_mut(self, lifetime);
    }

    fn visit_type_reference_mut(&mut self, type_reference: &mut TypeReference) {
        match self.replacement_strategy {
            ReplacementStrategy::ReplaceAll => {
                type_reference.lifetime = Some(self.new_lifetime.clone())
            }
            ReplacementStrategy::ReplaceOnlyOptional if type_reference.lifetime.is_none() => {
                type_reference.lifetime = Some(self.new_lifetime.clone())
            }
            _ => {}
        }

        visit_mut::visit_type_reference_mut(self, type_reference);
    }
}

pub(crate) enum ReplacementStrategy {
    ReplaceAll,
    ReplaceOnlyOptional,
}
