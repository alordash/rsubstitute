use crate::args_matching::ArgInfo;

pub struct FnCallInfo<'a> {
    pub(crate) fn_name: &'static str,
    pub(crate) call_args: Vec<ArgInfo<'a>>,
}

impl<'a> FnCallInfo<'a> {
    pub fn new(fn_name: &'static str, call_args: Vec<ArgInfo<'a>>) -> Self {
        FnCallInfo { fn_name, call_args }
    }
}
