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

    pub fn get_ptrs_info_suffix(&self, actual_value: &T) -> PtrInfo {
        self.maybe_deref_info
            .as_ref()
            .map(|deref_info| {
                let expected_ptr = deref_info.expected_value_deref_ptr;
                let actual_ptr = deref_info.get_actual_value_deref_ptr(actual_value);
                return PtrInfo {
                    expected_ptr_info_suffix: Self::format_ptr_info(expected_ptr),
                    actual_ptr_info_suffix: Self::format_ptr_info(actual_ptr),
                };
            })
            .unwrap_or_else(PtrInfo::empty)
    }

    fn format_ptr_info(ptr: *const ()) -> String {
        format!(" (ptr: {ptr:?})")
    }
}

pub(crate) struct PtrInfo {
    pub expected_ptr_info_suffix: String,
    pub actual_ptr_info_suffix: String,
}

impl PtrInfo {
    pub fn empty() -> Self {
        Self {
            expected_ptr_info_suffix: String::new(),
            actual_ptr_info_suffix: String::new(),
        }
    }
}
