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
        spans: fn_syntax.spans,
        attributes: fn_syntax.attributes,
        source_signature: fn_syntax.source_signature,
        visibility: fn_syntax.visibility,
        merged_generics: fn_syntax.merged_generics,
        generics_field: fn_syntax.generics_field,
        fn_ident: fn_syntax.fn_ident,
        maybe_self_type: fn_syntax.maybe_self_type,
        arguments: fn_syntax.arguments,
        arg_refs_tuple: fn_syntax.arg_refs_tuple,
        maybe_base_impl: fn_syntax.maybe_base_impl,
        return_type: fn_syntax.return_type,
        call_struct,
        args_checker_struct,
    };

    return result;
}
