pub trait IArgsTupleProvider {
    fn provide_ptr_to_tuple_of_refs(&self) -> *const ();
}
