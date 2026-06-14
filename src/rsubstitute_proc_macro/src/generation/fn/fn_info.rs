use crate::generation::r#fn::models::*;
use crate::generation::r#fn::*;
use crate::preparation::r#fn::models::*;

pub(crate) fn generate(fn_syntax: FnSyntax) -> FnInfo {
    let call_struct = call_struct::generate(&fn_syntax);
    let args_checker_struct = args_checker_struct::generate(&fn_syntax, call_struct.r#type.clone());

    let result = FnInfo {
        syntax: fn_syntax,
        call_struct,
        args_checker_struct,
    };

    return result;
}
