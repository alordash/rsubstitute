use syn::*;

pub(crate) trait IGenericsExtensions {
    fn with_head_param(self, param: GenericParam) -> Self;
}

impl IGenericsExtensions for Generics {
    fn with_head_param(mut self, param: GenericParam) -> Self {
        self.lt_token.get_or_insert(Default::default());
        self.gt_token.get_or_insert(Default::default());
        self
            .params
            .insert(0, param);
        return self;
    }
}