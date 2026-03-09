use crate::args::ArgCheckResult;
use crate::mock_data::FnConfig;
use std::cell::RefCell;
use std::sync::Arc;

pub(crate) enum MatchingConfigSearchResult<'rs, TMock> {
    Ok(Arc<RefCell<FnConfig<'rs, TMock>>>),
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
