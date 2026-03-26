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
        for other_lifetime_bound in other.bounds {
            if !self.contains_lifetime_bound(&other_lifetime_bound) {
                self.lifetime_param.bounds.push(other_lifetime_bound)
            }
        }
    }

    fn contains_lifetime_bound(&self, lifetime_bound: &Lifetime) -> bool {
        self.lifetime_param
            .bounds
            .iter()
            .any(|bound| bound.ident == lifetime_bound.ident)
    }
}

impl TypeParamMerger {
    pub fn merge(&mut self, other: TypeParam) {
        if self.type_param.default.is_some() && other.default.is_some() {
            panic!(
                "Encountered conflicting generic type parameter default value. TODO write more decriptive error message and make compile fail test"
            );
        }
    }
}

impl ConstParamMerger {
    pub fn merge(&mut self, other: ConstParam) {
        todo!()
    }
}
