use crate::IGenericsHashKeyProvider;
use crate::args::*;
use crate::fn_parameters::DynCall;

pub trait IArgsChecker: IArgsFormatter + IGenericsHashKeyProvider {
    fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult>;
}
