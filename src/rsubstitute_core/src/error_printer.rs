use crate::args_matching::{ArgCheckResult, ArgInfo, IArgsFormatter};
use crate::call_info::CallInfo;
use crate::{FnCallInfo, Times};

pub trait IErrorPrinter {
    fn print_received_verification_error(
        &self,
        fn_name: &'static str,
        args_formatter: &dyn IArgsFormatter,
        matching_calls: Vec<Vec<ArgCheckResult>>,
        non_matching_calls: Vec<Vec<ArgCheckResult>>,
        times: Times,
    ) -> !;

    fn print_received_unexpected_calls_error(&self, unexpected_calls: Vec<FnCallInfo>) -> !;
}

pub(crate) struct ErrorPrinter;

impl IErrorPrinter for ErrorPrinter {
    fn print_received_verification_error(
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
            r#"{times} matching:
{expected_call_msg}
{matching_calls_report}
{non_matching_calls_report}"#
        );
        panic!("{error_msg}");
    }

    fn print_received_unexpected_calls_error(&self, unexpected_calls: Vec<FnCallInfo>) -> ! {
        let unexpected_calls_count = unexpected_calls.len();
        let call_fmt = self.fmt_calls(unexpected_calls_count);
        let unexpected_calls_msgs: Vec<_> = unexpected_calls
            .into_iter()
            .enumerate()
            .map(|(i, fn_call_info)| {
                let call_number = i + 1;
                let fn_name = fn_call_info.fn_name;
                let call_args = fn_call_info.call_args;
                let call_args_msgs: Vec<_> = call_args
                    .into_iter()
                    .map(|call_arg| call_arg.arg_name())
                    .collect();
                let call_args_msg = call_args_msgs.join(", ");
                let unexpected_calls_msg = format!("{call_number}. {fn_name}({call_args_msg})");
                return unexpected_calls_msg;
            })
            .collect();
        let unexpected_calls_msg = unexpected_calls_msgs.join("\n");
        let error_msg = format!(
            r"Did not expect to receive ant other calls. Received {unexpected_calls_count} {call_fmt}:
{unexpected_calls_msg}"
        );
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
        format!("{fn_name}({args_msgs_joined}){errors_report}")
    }

    fn fmt_calls(&self, calls_count: usize) -> &'static str {
        assert_ne!(calls_count, 0);
        return if calls_count == 1 { "call" } else { "calls" };
    }
}
