use crate::args::*;
use crate::fn_parameters::DynArgsChecker;
use crate::matching_config_search_result::MatchingConfigSearchErr;
use crate::*;

pub(crate) fn panic_received_verification_error(
    fn_name: &'static str,
    args_formatter: &DynArgsChecker,
    matching_calls_check_result: CallsCheckResult,
    non_matching_calls_check_result: CallsCheckResult,
    times: Times,
) -> ! {
    let matching_calls_count = matching_calls_check_result.calls_args_check_results.len();

    let generic_parameter_infos = args_formatter.get_generic_parameter_infos();
    let generic_parameters_msg = fmt_generic_parameter_infos(GenericParameterInfosFormatting::Do(
        &generic_parameter_infos,
    ));
    let expected_call_msg = format!(
        "\t{fn_name}{}({})",
        generic_parameters_msg,
        args_formatter.fmt_args(),
    );
    let matching_calls_report = if matching_calls_count == 0 {
        "Actually received no matching calls".to_string()
    } else {
        let matching_calls_args_msgs: Vec<_> = matching_calls_check_result
            .calls_args_check_results
            .into_iter()
            .map(|x| fmt_call(fn_name, x, GenericParameterInfosFormatting::Ignore))
            .collect();
        let matching_calls_args_msg = matching_calls_args_msgs.join("\n\t");
        let call_fmt = fmt_calls(matching_calls_count);
        format!(
            "Actually received {matching_calls_count} matching {call_fmt}:
\t{matching_calls_args_msg}"
        )
    };
    let non_matching_calls_count = non_matching_calls_check_result
        .calls_args_check_results
        .len();
    let non_matching_calls_report = if non_matching_calls_count == 0 {
        "Received no non-matching calls".to_string()
    } else {
        let max_invalid_calls_listed_count = read_config().max_invalid_calls_listed_count;
        let call_fmt = fmt_calls(non_matching_calls_count);
        let non_matching_calls_args_msgs: Vec<_> = non_matching_calls_check_result
            .calls_args_check_results
            .into_iter()
            .take(max_invalid_calls_listed_count)
            .map(|x| fmt_call(fn_name, x, GenericParameterInfosFormatting::Ignore))
            .collect();
        let trimmed_output_disclaimer = if non_matching_calls_count > max_invalid_calls_listed_count
        {
            format!(" (listing only first {})", max_invalid_calls_listed_count)
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

pub(crate) fn panic_no_suitable_fn_configuration_found(
    fn_name: &'static str,
    unexpected_call: Vec<ArgInfo>,
    generic_parameter_infos: Vec<GenericParameterInfo>,
    matching_config_search_err: MatchingConfigSearchErr,
) -> ! {
    let call_msg =
        format_received_unexpected_call_error(fn_name, unexpected_call, generic_parameter_infos);
    let calls = matching_config_search_err
        .args_check_results_sorted_by_number_of_correctly_matched_args_descending;
    let configs_report = if calls.calls_args_check_results.len() > 0 {
        let args_check_results_msgs: Vec<_> = calls
            .calls_args_check_results
            .into_iter()
            .enumerate()
            .map(|(i, args_check_result)| {
                let number = i + 1;
                let matched_arguments_count = args_check_result.iter().filter(|x| x.is_ok()).count();
                let total_arguments_count = args_check_result.len();
                let args_msg = fmt_fn_parameters_msg(fn_name, args_check_result, GenericParameterInfosFormatting::Ignore);
                return format!("{number}. Matched {matched_arguments_count}/{total_arguments_count} arguments: {args_msg}");
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

pub(crate) fn format_received_unexpected_call_error(
    fn_name: &'static str,
    call_args: Vec<ArgInfo>,
    generic_parameter_infos: Vec<GenericParameterInfo>,
) -> String {
    let call_args_msgs: Vec<_> = call_args
        .into_iter()
        .map(|call_arg| call_arg.clone_arg_debug_string())
        .collect();
    let call_args_msg = call_args_msgs.join(", ");
    let generic_parameters_msg = fmt_generic_parameter_infos(GenericParameterInfosFormatting::Do(
        &generic_parameter_infos,
    ));
    let error_msg = format!("{fn_name}{generic_parameters_msg}({call_args_msg})");
    return error_msg;
}

pub(crate) fn panic_received_unexpected_calls_error(error_msgs: Vec<String>) -> ! {
    let unexpected_calls_count = error_msgs.len();
    let call_fmt = fmt_calls(unexpected_calls_count);
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
        "Did not expect to receive any other calls. Received {unexpected_calls_count} unexpected {call_fmt}:
{unexpected_calls_msg}"
    );
    panic!("{error_msg}");
}

pub(crate) fn panic_no_return_value_was_configured(
    fn_name: &'static str,
    call_args: Vec<ArgInfo>,
    generic_parameter_infos: Vec<GenericParameterInfo>,
) -> ! {
    let call_msg =
        format_received_unexpected_call_error(fn_name, call_args, generic_parameter_infos);
    let error_msg = format!("No return value found for following call: {call_msg}");
    panic!("{error_msg}");
}

fn fmt_call(
    fn_name: &'static str,
    args_check_results: Vec<ArgCheckResult>,
    generic_parameter_infos_formatting: GenericParameterInfosFormatting,
) -> String {
    let error_msgs: Vec<_> = args_check_results
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
    let args_msg = fmt_fn_parameters_msg(
        fn_name,
        args_check_results,
        generic_parameter_infos_formatting,
    );
    format!("{args_msg}{errors_report}")
}

fn fmt_fn_parameters_msg(
    fn_name: &'static str,
    args_check_results: Vec<ArgCheckResult>,
    generic_parameter_infos_formatting: GenericParameterInfosFormatting,
) -> String {
    let args_msgs: Vec<_> = args_check_results
        .into_iter()
        .map(|x| match x {
            ArgCheckResult::Ok(x) => x.arg_info.clone_arg_debug_string(),
            ArgCheckResult::Err(x) => format!("*{}*", x.arg_info.clone_arg_debug_string()),
        })
        .collect();
    let args_msgs_joined = args_msgs.join(", ");
    let generic_parameters_msg = fmt_generic_parameter_infos(generic_parameter_infos_formatting);
    let args_msg = format!("{fn_name}{generic_parameters_msg}({args_msgs_joined})");
    return args_msg;
}

fn fmt_generic_parameter_infos(
    generic_parameter_infos_formatting: GenericParameterInfosFormatting,
) -> String {
    let result = match generic_parameter_infos_formatting {
        GenericParameterInfosFormatting::Do(generic_parameter_infos)
            if !generic_parameter_infos.is_empty() =>
        {
            let generic_parameters_msgs: Vec<_> = generic_parameter_infos
                .into_iter()
                .map(|x| x.to_string())
                .collect();
            let generic_parameters_msgs_joined = generic_parameters_msgs.join(", ");
            format!("<{generic_parameters_msgs_joined}>")
        }
        _ => String::new(),
    };
    return result;
}

fn fmt_calls(calls_count: usize) -> &'static str {
    assert_ne!(calls_count, 0);
    return if calls_count == 1 { "call" } else { "calls" };
}

enum GenericParameterInfosFormatting<'a> {
    Do(&'a [GenericParameterInfo]),
    Ignore,
}
