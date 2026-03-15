use crate::args::*;
use crate::fn_parameters::*;
use crate::*;
use std::ops::Deref;

pub struct DynCall<'rs> {
    inner: Box<dyn ICall + 'rs>,
}

impl<'rs> IArgsInfosProvider for DynCall<'rs> {
    fn get_arg_infos(&self) -> Vec<ArgInfo> {
        self.inner.get_arg_infos()
    }
}

impl<'rs> IGenericsInfoProvider for DynCall<'rs> {
    fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
        self.inner.get_generic_parameter_infos()
    }


    fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {
        self.inner.hash_generics_type_ids(hasher)
    }

    fn hash_const_values(&self, hasher: &mut GenericsHasher) {
        self.inner.hash_const_values(hasher)
    }
}

impl<'rs> IArgsTupleProvider for DynCall<'rs> {
    fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
        self.inner.get_ptr_to_boxed_tuple_of_refs()
    }
}

impl<'rs> DynCall<'rs> {
    pub fn new<'a, T: ICall + 'rs>(value: T) -> DynCall<'a> {
        transmute_lifetime!(Self {
            inner: Box::new(value),
        })
    }

    pub fn downcast_ref<T: 'rs>(&self) -> &T {
        let dyn_ref = self.inner.as_ref();
        // SAFETY: for justification refer to module level documentation.
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
