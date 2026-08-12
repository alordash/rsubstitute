use crate::args::*;
use crate::fn_parameters::DynCall;

pub trait IArgsChecker: IGenericsInfoProvider {
    fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult>;

    fn fmt_args(&self) -> String;
}
