use crate::FnConfig;
use crate::args::ArgCheckResult;
use std::cell::RefCell;
use std::sync::Arc;

pub(crate) enum MatchingConfigSearchResult<TMock, TCall, TReturnType, TArgsChecker> {
    Ok(Arc<RefCell<FnConfig<TMock, TCall, TReturnType, TArgsChecker>>>),
    Err(MatchingConfigSearchErr),
}

pub(crate) struct MatchingConfigSearchErr {
    pub args_check_results_sorted_by_number_of_correctly_matched_args_descending:
        Vec<Vec<ArgCheckResult>>,
}

impl MatchingConfigSearchErr {
    pub fn empty() -> Self {
        Self {
            args_check_results_sorted_by_number_of_correctly_matched_args_descending: Vec::new(),
        }
    }
}
