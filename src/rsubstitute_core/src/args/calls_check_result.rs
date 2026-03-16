use crate::args::*;

pub(crate) struct CallsCheckResult {
    pub calls_args_check_results: Vec<Vec<ArgCheckResult>>,
    pub generic_parameter_infos: Vec<GenericParameterInfo>,
}

impl CallsCheckResult {
    pub fn new(
        calls_args_check_results: Vec<Vec<ArgCheckResult>>,
        generic_parameter_infos: Vec<GenericParameterInfo>,
    ) -> Self {
        Self {
            calls_args_check_results,
            generic_parameter_infos,
        }
    }

    pub fn empty() -> Self {
        Self {
            calls_args_check_results: Vec::new(),
            generic_parameter_infos: Vec::new()
        }
    }
}
