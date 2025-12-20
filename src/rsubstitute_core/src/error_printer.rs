use crate::Times;
use crate::args_matching::{ArgMatchingResult, IArgsMatcher};

pub trait IErrorPrinter {
    fn print_received_verification_error<TCall, TArgsMatcher: IArgsMatcher<TCall>>(
        &self,
        fn_name: &'static str,
        args_matcher: TArgsMatcher,
        matching_calls: Vec<Vec<ArgMatchingResult>>,
        not_matching_calls: Vec<Vec<ArgMatchingResult>>,
        times: Times,
    ) -> !;
}

pub struct ErrorPrinter;

impl IErrorPrinter for ErrorPrinter {
    fn print_received_verification_error<TCall, TArgsMatcher: IArgsMatcher<TCall>>(
        &self,
        fn_name: &'static str,
        args_matcher: TArgsMatcher,
        matching_calls: Vec<Vec<ArgMatchingResult>>,
        not_matching_calls: Vec<Vec<ArgMatchingResult>>,
        times: Times,
    ) -> ! {
        let matching_calls_count = matching_calls.len();
        debug_assert!(matching_calls_count != 0);

        let expected_call_str = format!("\t{fn_name}({args_matcher:?})");
        let matching_calls_str = if matching_calls_count == 0 {
            "Actually received no matching calls".to_string()
        } else {
            format!(
                r#"Actually received {matching_calls_count} matching calls:
\t"#
            )
        };
        let msg = format!(
            r#"Expected to receive a call {times} matching:
{expected_call_str}
Actually received {matching_calls_count} matching calls.
Received "#
        );
        todo!()
    }
}

impl ErrorPrinter {
    fn fmt_call(&self, call: Vec<ArgMatchingResult>) -> String {
        todo!()
        // format!()
    }
}