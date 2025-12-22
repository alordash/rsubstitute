use crate::args_matching::{ArcCheckResult, IArgsFormatter};

pub trait IArgsChecker<TCall>: IArgsFormatter {
    fn check(&self, call: TCall) -> Vec<ArcCheckResult>;
}
