use crate::generation::r#fn::models::*;
use crate::generation::r#fn::*;
use crate::preparation::r#fn::models::*;

pub(crate) fn generate_fn_info(fn_syntax: FnSyntax) -> FnInfo {
    let call_struct = generate_call_struct(&fn_syntax);
    let args_checker_struct = generate_args_checker_struct(&fn_syntax, call_struct.r#type.clone());

    let result = FnInfo {
        syntax: fn_syntax,
        call_struct,
        args_checker_struct,
    };

    return result;
}
