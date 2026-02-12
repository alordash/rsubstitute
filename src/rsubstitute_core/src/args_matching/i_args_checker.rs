use crate::args_matching::{ArgCheckResult, IArgsFormatter};
use crate::fn_parameters::IRawCall;
// pub trait IArgsChecker<TCall>: IArgsFormatter {
//     fn check(&self, call: &TCall) -> Vec<ArgCheckResult>;
// }

pub trait IDynArgsChecker: IArgsFormatter {
    fn check(&self, raw_call: &dyn IRawCall) -> Vec<ArgCheckResult>;
}