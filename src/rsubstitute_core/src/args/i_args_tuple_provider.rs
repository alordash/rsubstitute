pub trait IArgsTupleProvider {
    fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut ();
}
