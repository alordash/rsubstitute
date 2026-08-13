use crate::args::*;
use crate::fn_parameters::DynCall;

pub trait IArgsChecker: IGenericsInfoProvider {
    fn check(&self, #[allow(unused_variables)] dyn_call: &DynCall) -> Vec<ArgCheckResult> {
        Vec::new()
    }

    fn fmt_args(&self) -> String {
        String::new()
    }
}
