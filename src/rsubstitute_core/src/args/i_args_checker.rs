use crate::IGenericsHashKeyProvider;
use crate::args::*;

pub trait IArgsChecker<TCall>: IArgsFormatter + IGenericsHashKeyProvider {
    fn check(&self, call: &TCall) -> Vec<ArgCheckResult>;
}
