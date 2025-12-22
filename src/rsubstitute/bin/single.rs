use crate::IFoo;
use rsubstitute_core::args_matching::{Arg, IArgsFormatter};
use rsubstitute_proc_macro::IArgsFormatter;
use std::sync::Arc;

#[allow(non_camel_case_types)]
#[derive(Debug, IArgsFormatter)]
pub struct another_work_ArgsChecker<'a> {
    pub string: Arg<&'a str>,
    pub something: Arg<&'a &'a [u8]>,
    pub dyn_obj: Arg<&'a dyn IFoo>,
    pub arc: Arg<Arc<dyn IFoo>>,
}
