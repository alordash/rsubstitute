use crate::args::*;
use crate::fn_parameters::{IArgsTupleProvider, ICall};
use crate::{GenericsHasher, IGenericsHashKeyProvider};
use std::ops::Deref;

pub struct DynCall<'rs> {
    inner: Box<dyn ICall + 'rs>,
}

impl<'rs> IArgInfosProvider for DynCall<'rs> {
    fn get_arg_infos(&self) -> Vec<ArgInfo> {
        self.inner.get_arg_infos()
    }
}

impl<'rs> IGenericsHashKeyProvider for DynCall<'rs> {
    fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {
        self.inner.hash_generics_type_ids(hasher)
    }

    fn hash_const_values(&self, hasher: &mut GenericsHasher) {
        self.inner.hash_const_values(hasher)
    }
}

impl<'rs> IArgsTupleProvider for DynCall<'rs> {
    fn provide_ptr_to_tuple_of_refs(&self) -> *const () {
        self.inner.provide_ptr_to_tuple_of_refs()
    }
}

impl<'rs> DynCall<'rs> {
    pub fn new<T: ICall + 'rs>(value: T) -> Self {
        Self {
            inner: Box::new(value),
        }
    }

    pub fn downcast_ref<T: 'rs>(&self) -> &T {
        let dyn_ref = self.inner.as_ref();
        let t_ref = unsafe { &*(dyn_ref as *const _ as *const T) };
        return t_ref;
    }
}

impl<'rs> Deref for DynCall<'rs> {
    type Target = dyn ICall + 'rs;

    fn deref(&self) -> &Self::Target {
        return self.inner.as_ref();
    }
}
