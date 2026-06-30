use crate::common::models::*;
use crate::generation::fn_info::models::*;
use crate::generation::fn_info::*;
use crate::preparation::r#fn::models::*;
use syn::*;

pub(crate) fn generate(ctx: &Context, fn_syntax: FnSyntax) -> FnInfo {
    let call_struct = call_struct::generate(ctx, &fn_syntax);
    let args_checker_struct = args_checker_struct::generate(
        &fn_syntax,
        Type::Path(TypePath {
            qself: None,
            path: call_struct.path.clone(),
        }),
    );

    let result = FnInfo {
        syntax: fn_syntax,
        call_struct,
        args_checker_struct,
    };

    return result;
}
