use crate::args::*;
use crate::fn_parameters::DynCall;

pub trait IArgsChecker: IArgsFormatter + IGenericsInfoProvider {
    fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult>;
}
