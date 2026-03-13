use syn::*;

pub(crate) trait IGenericsExtensions {
    fn with_head_lifetime_param(self, lifetime_param: LifetimeParam) -> Self;
}

impl IGenericsExtensions for Generics {
    fn with_head_lifetime_param(mut self, lifetime_param: LifetimeParam) -> Self {
        self.lt_token.get_or_insert(Default::default());
        self.gt_token.get_or_insert(Default::default());
        self.params
            .insert(0, GenericParam::Lifetime(lifetime_param));
        return self;
    }
}
