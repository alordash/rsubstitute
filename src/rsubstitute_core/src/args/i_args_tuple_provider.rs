use crate::fn_parameters::DynArgRefsTuple;

pub trait IArgsTupleProvider {
    fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut ();

    fn get_dyn_tuple_of_refs<'a>(&self) -> DynArgRefsTuple<'a> {
        let raw_ptr = self.get_ptr_to_boxed_tuple_of_refs();
        return DynArgRefsTuple::from_raw(raw_ptr);
    }
}
