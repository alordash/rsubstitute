use crate::args::DerefInfo;

#[repr(C)]
pub(crate) struct ArgCmp<T> {
    pub value: T,
    pub comparator: fn(&T, &T) -> bool,
    pub maybe_deref_info: Option<DerefInfo>,
}

impl<T> ArgCmp<T> {
    pub fn is_arg_equal_to(&self, other: &T) -> bool {
        (self.comparator)(&self.value, other)
    }
}
