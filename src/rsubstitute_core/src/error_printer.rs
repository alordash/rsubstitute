use crate::Times;
use crate::args_matching::{ArgCheckResult, ArgInfo, IArgsChecker, IArgsFormatter};
use crate::matching_config_search_result::MatchingConfigSearchErr;

pub(crate) trait IErrorPrinter {
    fn panic_received_verification_error(
        &self,
        fn_name: &'static str,
        args_formatter: &dyn IArgsFormatter,
        matching_calls: Vec<Vec<ArgCheckResult>>,
        non_matching_calls: Vec<Vec<ArgCheckResult>>,
        times: Times,
    ) -> !;

    fn panic_no_suitable_fn_configuration_found(
        &self,
        fn_name: &'static str,
        unexpected_call: Vec<ArgInfo>,
        matching_config_search_err: MatchingConfigSearchErr,
    ) -> !;

    fn format_received_unexpected_call_error(
        &self,
        fn_name: &'static str,
        unexpected_call: Vec<ArgInfo>,
    ) -> String;

    fn panic_received_unexpected_calls_error(&self, error_msgs: Vec<String>) -> !;

    fn panic_no_return_value_was_configured(
        &self,
        fn_name: &'static str,
        call_args: Vec<ArgInfo>,
    ) -> !;
}

pub(crate) struct ErrorPrinter;

impl IErrorPrinter for ErrorPrinter {
    fn panic_received_verification_error(
        &self,
        fn_name: &'static str,
        args_formatter: &dyn IArgsFormatter,
        matching_calls: Vec<Vec<ArgCheckResult>>,
        non_matching_calls: Vec<Vec<ArgCheckResult>>,
        times: Times,
    ) -> ! {
        let matching_calls_count = matching_calls.len();

        let expected_call_msg = format!("\t{fn_name}({})", args_formatter.fmt_args());
        let matching_calls_report = if matching_calls_count == 0 {
            "Actually received no matching calls".to_string()
        } else {
            let matching_calls_args_msgs: Vec<_> = matching_calls
                .into_iter()
                .map(|x| self.fmt_call(fn_name, x))
                .collect();
            let matching_calls_args_msg = matching_calls_args_msgs.join("\n\t");
            let call_fmt = self.fmt_calls(matching_calls_count);
            format!(
                "Actually received {matching_calls_count} matching {call_fmt}:
\t{matching_calls_args_msg}"
            )
        };
        let non_matching_calls_count = non_matching_calls.len();
        let non_matching_calls_report = if non_matching_calls_count == 0 {
            "Received no non-matching calls".to_string()
        } else {
            let call_fmt = self.fmt_calls(non_matching_calls_count);
            let non_matching_calls_args_msgs: Vec<_> = non_matching_calls
                .into_iter()
                .take(Self::MAX_INVALID_CALLS_LISTED_COUNT)
                .map(|x| self.fmt_call(fn_name, x))
                .collect();
            let trimmed_output_disclaimer =
                if non_matching_calls_count > Self::MAX_INVALID_CALLS_LISTED_COUNT {
                    format!(
                        " (listing only first {})",
                        Self::MAX_INVALID_CALLS_LISTED_COUNT
                    )
                } else {
                    String::new()
                };
            let non_matching_calls_args_msg = non_matching_calls_args_msgs.join("\n");
            format!(
                "Received {non_matching_calls_count} non-matching {call_fmt}{trimmed_output_disclaimer} (non-matching arguments indicated with '*' characters):
{non_matching_calls_args_msg}"
            )
        };
        let error_msg = format!(
            r"{times} matching:
{expected_call_msg}
{matching_calls_report}
{non_matching_calls_report}"
        );
        panic!("{error_msg}");
    }

    fn panic_no_suitable_fn_configuration_found(
        &self,
        fn_name: &'static str,
        unexpected_call: Vec<ArgInfo>,
        matching_config_search_err: MatchingConfigSearchErr,
    ) -> ! {
        let call_msg = self.format_received_unexpected_call_error(fn_name, unexpected_call);
        let configs_report = if matching_config_search_err
            .args_check_results_sorted_by_number_of_correctly_matched_args_descending
            .len()
            > 0
        {
            let args_check_results_msgs: Vec<_> = matching_config_search_err
                .args_check_results_sorted_by_number_of_correctly_matched_args_descending
                .into_iter()
                .enumerate()
                .map(|(i, args_check_result)| {
                    let number = i + 1;
                    let args_msg = self.fmt_args_msg(fn_name, args_check_result);
                    return format!("{number}. {args_msg}");
                })
                .collect();
            let args_check_results_msg = args_check_results_msgs.join("\n\t");
            format!(
                "
List of existing configuration ordered by number of correctly matched arguments (non-matching arguments indicated with '*' characters):
\t{args_check_results_msg}"
            )
        } else {
            String::new()
        };
        let error_msg = format!(
            "Mock wasn't configured to handle following call:
\t{call_msg}{configs_report}"
        );
        panic!("{error_msg}");
    }

    fn format_received_unexpected_call_error(
        &self,
        fn_name: &'static str,
        call_args: Vec<ArgInfo>,
    ) -> String {
        let call_args_msgs: Vec<_> = call_args
            .into_iter()
            .map(|call_arg| format!("{:?}", call_arg.arg_value()))
            .collect();
        let call_args_msg = call_args_msgs.join(", ");
        let error_msg = format!("{fn_name}({call_args_msg})");
        return error_msg;
    }

    fn panic_received_unexpected_calls_error(&self, error_msgs: Vec<String>) -> ! {
        let unexpected_calls_count = error_msgs.len();
        let call_fmt = self.fmt_calls(unexpected_calls_count);
        let unexpected_calls_msgs: Vec<_> = error_msgs
            .into_iter()
            .enumerate()
            .map(|(i, error_msg)| {
                let error_number = i + 1;
                return format!("{error_number}. {error_msg}");
            })
            .collect();
        let unexpected_calls_msg = unexpected_calls_msgs.join("\n");
        let error_msg = format!(
            "Did not expect to receive any other calls. Received {unexpected_calls_count} {call_fmt}:
{unexpected_calls_msg}"
        );
        panic!("{error_msg}");
    }

    fn panic_no_return_value_was_configured(
        &self,
        fn_name: &'static str,
        call_args: Vec<ArgInfo>,
    ) -> ! {
        let call_msg = self.format_received_unexpected_call_error(fn_name, call_args);
        let error_msg = format!("No return value was configured for following call: {call_msg}");
        panic!("{error_msg}");
    }
}

impl ErrorPrinter {
    // TODO - should be configurable
    const MAX_INVALID_CALLS_LISTED_COUNT: usize = 10;

    fn fmt_call(&self, fn_name: &'static str, call: Vec<ArgCheckResult>) -> String {
        let error_msgs: Vec<_> = call
            .iter()
            .filter_map(ArgCheckResult::as_err)
            .enumerate()
            .map(|(i, x)| {
                let error_number = i + 1;
                format!(
                    "{}. {} ({}):
{}",
                    error_number,
                    x.arg_info.arg_name(),
                    x.arg_info.arg_type_name(),
                    x.error_msg
                )
            })
            .collect();
        let errors_count = error_msgs.len();
        let errors_report = if errors_count == 0 {
            String::new()
        } else {
            let error_msgs_joined = error_msgs.join("\n\t");
            format!(
                "
\t{error_msgs_joined}"
            )
        };
        let args_msg = self.fmt_args_msg(fn_name, call);
        format!("{args_msg}{errors_report}")
    }

    fn fmt_args_msg(&self, fn_name: &'static str, call: Vec<ArgCheckResult>) -> String {
        let args_msgs: Vec<_> = call
            .into_iter()
            .map(|x| match x {
                ArgCheckResult::Ok(x) => {
                    format!("{:?}", x.arg_info.arg_value())
                }
                ArgCheckResult::Err(x) => {
                    format!("*{:?}*", x.arg_info.arg_value())
                }
            })
            .collect();
        let args_msgs_joined = args_msgs.join(", ");
        let args_msg = format!("{fn_name}({args_msgs_joined})");
        return args_msg;
    }

    fn fmt_calls(&self, calls_count: usize) -> &'static str {
        assert_ne!(calls_count, 0);
        return if calls_count == 1 { "call" } else { "calls" };
    }
}
