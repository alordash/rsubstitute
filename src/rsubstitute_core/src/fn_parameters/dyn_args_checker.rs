use crate::args::*;
use crate::fn_parameters::DynCall;
use crate::*;
use std::ops::Deref;

pub struct DynArgsChecker<'rs> {
    inner: Box<dyn IArgsChecker + 'rs>,
}

impl<'rs> IArgsFormatter for DynArgsChecker<'rs> {
    fn fmt_args(&self) -> String {
        self.inner.fmt_args()
    }
}

impl<'rs> IGenericsHashKeyProvider for DynArgsChecker<'rs> {
    fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {
        self.inner.hash_generics_type_ids(hasher)
    }

    fn hash_const_values(&self, hasher: &mut GenericsHasher) {
        self.inner.hash_const_values(hasher)
    }
}

impl<'rs> IArgsChecker for DynArgsChecker<'rs> {
    fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
        self.inner.check(dyn_call)
    }
}

impl<'rs> DynArgsChecker<'rs> {
    pub fn new<T: IArgsChecker + 'rs>(value: T) -> Self {
        Self {
            inner: Box::new(value),
        }
    }
}

impl<'rs> Deref for DynArgsChecker<'rs> {
    type Target = dyn IArgsChecker + 'rs;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}
