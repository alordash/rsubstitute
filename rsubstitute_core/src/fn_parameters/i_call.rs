use crate::args::*;
use crate::fn_parameters::*;

pub trait ICall: IGenericsInfoProvider {
    fn get_arg_infos(&self) -> Vec<ArgInfo> {
        Vec::new()
    }

    fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
        Box::leak(Box::new(())) as *mut _
    }

    #[doc(hidden)]
    #[allow(private_interfaces)]
    fn get_dyn_tuple_of_refs<'a>(&self) -> DynArgRefsTuple<'a> {
        let raw_ptr = self.get_ptr_to_boxed_tuple_of_refs();
        return DynArgRefsTuple::from_raw(raw_ptr);
    }
}
