use crate::FnConfig;
use crate::args_matching::ArgCheckResult;
use std::cell::RefCell;
use std::sync::Arc;

pub(crate) enum MatchingConfigSearchResult<TMock, TArgsChecker> {
    Ok(Arc<RefCell<FnConfig<TMock, TArgsChecker>>>),
    Err(MatchingConfigSearchErr),
}

pub(crate) struct MatchingConfigSearchErr {
    pub args_check_results_sorted_by_number_of_correctly_matched_args_descending:
        Vec<Vec<ArgCheckResult>>,
}
