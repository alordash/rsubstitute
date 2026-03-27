use quote::ToTokens;
use syn::*;

pub(crate) enum GenericParamMerger {
    Lifetime(LifetimeParamMerger),
    Type(TypeParamMerger),
    Const(ConstParamMerger),
}

impl From<LifetimeParamMerger> for GenericParamMerger {
    fn from(value: LifetimeParamMerger) -> Self {
        GenericParamMerger::Lifetime(value)
    }
}

impl From<TypeParamMerger> for GenericParamMerger {
    fn from(value: TypeParamMerger) -> Self {
        GenericParamMerger::Type(value)
    }
}

impl From<ConstParamMerger> for GenericParamMerger {
    fn from(value: ConstParamMerger) -> Self {
        GenericParamMerger::Const(value)
    }
}

impl GenericParamMerger {
    pub fn order(&self) -> usize {
        match self {
            GenericParamMerger::Lifetime(lifetime_param_merger) => lifetime_param_merger.order,
            GenericParamMerger::Type(type_param_merger) => type_param_merger.order,
            GenericParamMerger::Const(const_param_merger) => const_param_merger.order,
        }
    }
}

impl Into<GenericParam> for GenericParamMerger {
    fn into(self) -> GenericParam {
        match self {
            GenericParamMerger::Lifetime(lifetime_param_merger) => {
                GenericParam::Lifetime(lifetime_param_merger.lifetime_param)
            }
            GenericParamMerger::Type(type_param_merger) => {
                GenericParam::Type(type_param_merger.type_param)
            }
            GenericParamMerger::Const(const_param_merger) => {
                GenericParam::Const(const_param_merger.const_param)
            }
        }
    }
}

pub(crate) struct LifetimeParamMerger {
    pub order: usize,
    pub lifetime_param: LifetimeParam,
}

pub(crate) struct TypeParamMerger {
    pub order: usize,
    pub type_param: TypeParam,
}

pub(crate) struct ConstParamMerger {
    pub order: usize,
    pub const_param: ConstParam,
}

impl LifetimeParamMerger {
    pub fn merge(&mut self, other: LifetimeParam) {
        self.lifetime_param.bounds.extend(other.bounds);
    }
}

impl TypeParamMerger {
    pub fn merge(&mut self, other: TypeParam) {
        // TODO - write tests for cases with conflicts
        if let Some(ref existing) = self.type_param.default
            && let Some(ref new) = other.default
        {
            panic!(
                "Encountered conflicting generic type parameter default value. Type parameter: {}, existing default value: {}, new default value: {}",
                self.type_param.ident.to_string(),
                existing.to_token_stream().to_string(),
                new.to_token_stream().to_string()
            );
        }
        if other.default.is_some() {
            self.type_param.eq_token = other.eq_token;
            self.type_param.default = other.default;
        }
        self.type_param.bounds.extend(other.bounds);
    }
}

impl ConstParamMerger {
    pub fn merge(&mut self, other: ConstParam) {
        if let Some(ref existing) = self.const_param.default
            && let Some(ref new) = other.default
        {
            panic!(
                "Encountered conflicting generic const parameter default value. Const parameter: {}, existing default value: {}, new default value: {}",
                self.const_param.ident.to_string(),
                existing.to_token_stream().to_string(),
                new.to_token_stream().to_string()
            );
        }
        if other.default.is_some() {
            self.const_param.eq_token = other.eq_token;
            self.const_param.default = other.default;
        }
    }
}
